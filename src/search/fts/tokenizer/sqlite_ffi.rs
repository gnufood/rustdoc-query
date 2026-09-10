//! `SQLite`'s FTS5 tokenizer ABI.

#![allow(unsafe_code)]

use std::{
    ffi::{c_char, c_int, c_void, CStr},
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::{self, NonNull},
};

use tokio_rusqlite::rusqlite::{self, ffi};

use super::identifier::{tokenize_document, tokenize_query, Token};

const TOKENIZER_NAME: &CStr = c"rust_ident";
const FTS5_API_POINTER_TYPE: &CStr = c"fts5_api_ptr";
const FTS5_API_SQL: &CStr = c"SELECT fts5(?1)";

type TokenCallback =
    unsafe extern "C" fn(*mut c_void, c_int, *const c_char, c_int, c_int, c_int) -> c_int;

static IDENTIFIER_TOKENIZER: ffi::fts5_tokenizer_v2 = ffi::fts5_tokenizer_v2 {
    iVersion: 2,
    xCreate: Some(create),
    xDelete: Some(delete),
    xTokenize: Some(tokenize),
};

/// Register `rust_ident` on a `SQLite` connection.
///
/// Must be called before using an FTS5 table configured with `rust_ident`.
///
/// # Errors
///
/// Returns the `SQLite` error produced while registering the tokenizer.
pub(crate) fn register_rust_identifier_tokenizer(
    conn: &rusqlite::Connection,
) -> rusqlite::Result<()> {
    // SAFETY: `conn` remains borrowed until all raw SQLite calls complete.
    let db = unsafe { conn.handle() };
    let api = unsafe { fts5_api(db)? };
    unsafe { register(api) }
}

unsafe extern "C" fn create(
    _context: *mut c_void,
    _arguments: *mut *const c_char,
    _argument_count: c_int,
    output: *mut *mut ffi::Fts5Tokenizer,
) -> c_int {
    if output.is_null() {
        return ffi::SQLITE_MISUSE;
    }
    // The tokenizer has no state. SQLite still requires a non-null instance
    // pointer and later returns this exact allocation to `delete`.
    unsafe { output.write(Box::into_raw(Box::new(())).cast()) };
    ffi::SQLITE_OK
}

unsafe extern "C" fn delete(tokenizer: *mut ffi::Fts5Tokenizer) {
    if !tokenizer.is_null() {
        unsafe { drop(Box::from_raw(tokenizer.cast::<()>())) };
    }
}

unsafe extern "C" fn tokenize(
    _tokenizer: *mut ffi::Fts5Tokenizer,
    context: *mut c_void,
    flags: c_int,
    text: *const c_char,
    text_len: c_int,
    _locale: *const c_char,
    _locale_len: c_int,
    callback: Option<TokenCallback>,
) -> c_int {
    // Panicking across the extern "C" boundary is UB; convert it to SQLITE_ERROR instead.
    catch_unwind(AssertUnwindSafe(|| unsafe {
        tokenize_inner(context, flags, text, text_len, callback)
    }))
    .unwrap_or(ffi::SQLITE_ERROR)
}

unsafe fn tokenize_inner(
    context: *mut c_void,
    flags: c_int,
    text: *const c_char,
    text_len: c_int,
    callback: Option<TokenCallback>,
) -> c_int {
    let Some(callback) = callback else {
        return ffi::SQLITE_MISUSE;
    };
    let Ok(text) = (unsafe { decode_text(text, text_len) }) else {
        return ffi::SQLITE_MISUSE;
    };
    let tokens = if flags & ffi::FTS5_TOKENIZE_DOCUMENT != 0 {
        tokenize_document(text)
    } else {
        tokenize_query(text)
    };
    unsafe { emit_tokens(callback, context, tokens) }
}

unsafe fn emit_tokens(callback: TokenCallback, context: *mut c_void, tokens: Vec<Token>) -> c_int {
    for token in tokens {
        let Ok(length) = c_int::try_from(token.term.len()) else {
            return ffi::SQLITE_TOOBIG;
        };
        let Ok(start) = c_int::try_from(token.start) else {
            return ffi::SQLITE_TOOBIG;
        };
        let Ok(end) = c_int::try_from(token.end) else {
            return ffi::SQLITE_TOOBIG;
        };
        let flags = if token.colocated {
            ffi::FTS5_TOKEN_COLOCATED
        } else {
            0
        };
        let result = unsafe {
            callback(
                context,
                flags,
                token.term.as_ptr().cast(),
                length,
                start,
                end,
            )
        };
        if result != ffi::SQLITE_OK {
            return result;
        }
    }
    ffi::SQLITE_OK
}

/// Caller must not let the returned `&str` outlive `SQLite`'s buffer for this call.
unsafe fn decode_text<'a>(text: *const c_char, length: c_int) -> Result<&'a str, ()> {
    if text.is_null() {
        return Err(());
    }
    let length = usize::try_from(length).map_err(|_| ())?;
    let bytes = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), length) };
    std::str::from_utf8(bytes).map_err(|_| ())
}

unsafe fn fts5_api(db: *mut ffi::sqlite3) -> rusqlite::Result<NonNull<ffi::fts5_api>> {
    let db =
        NonNull::new(db).ok_or_else(|| sqlite_error(ffi::SQLITE_MISUSE, "null SQLite handle"))?;
    let mut api: *mut ffi::fts5_api = ptr::null_mut();
    let mut statement = ptr::null_mut();
    let result = unsafe {
        ffi::sqlite3_prepare_v2(
            db.as_ptr(),
            FTS5_API_SQL.as_ptr(),
            -1,
            &raw mut statement,
            ptr::null_mut(),
        )
    };
    if result != ffi::SQLITE_OK {
        return Err(sqlite_error(result, "prepare FTS5 API lookup"));
    }
    let statement = Statement(statement);
    let result = unsafe {
        ffi::sqlite3_bind_pointer(
            statement.0,
            1,
            ptr::addr_of_mut!(api).cast(),
            FTS5_API_POINTER_TYPE.as_ptr(),
            None,
        )
    };
    if result != ffi::SQLITE_OK {
        return Err(sqlite_error(result, "bind FTS5 API pointer"));
    }
    let result = unsafe { ffi::sqlite3_step(statement.0) };
    if result != ffi::SQLITE_ROW {
        return Err(sqlite_error(result, "read FTS5 API pointer"));
    }
    NonNull::new(api).ok_or_else(|| sqlite_error(ffi::SQLITE_ERROR, "FTS5 is unavailable"))
}

unsafe fn register(api: NonNull<ffi::fts5_api>) -> rusqlite::Result<()> {
    let create = unsafe { api.as_ref().xCreateTokenizer_v2 }
        .ok_or_else(|| sqlite_error(ffi::SQLITE_ERROR, "FTS5 v2 tokenizer API is unavailable"))?;
    let result = unsafe {
        create(
            api.as_ptr(),
            TOKENIZER_NAME.as_ptr(),
            ptr::null_mut(),
            (&raw const IDENTIFIER_TOKENIZER).cast_mut(),
            None,
        )
    };
    if result == ffi::SQLITE_OK {
        Ok(())
    } else {
        Err(sqlite_error(result, "register rust_ident tokenizer"))
    }
}

struct Statement(*mut ffi::sqlite3_stmt);

impl Drop for Statement {
    fn drop(&mut self) {
        unsafe { ffi::sqlite3_finalize(self.0) };
    }
}

fn sqlite_error(code: c_int, context: &str) -> rusqlite::Error {
    rusqlite::Error::SqliteFailure(ffi::Error::new(code), Some(context.to_owned()))
}
