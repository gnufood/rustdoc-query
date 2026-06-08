use super::{parts::emit_parts, Token};

pub(crate) fn tokenize_document(text: &str) -> Vec<Token> {
    tokenize(text, true)
}

pub(crate) fn tokenize_query(text: &str) -> Vec<Token> {
    tokenize(text, false)
}

fn tokenize(text: &str, include_parts: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut word_start = None;

    for (end, ch) in text
        .char_indices()
        .chain(std::iter::once((text.len(), '\0')))
    {
        if ch.is_alphanumeric() || ch == '_' {
            word_start.get_or_insert(end);
        } else if let Some(start) = word_start.take() {
            emit_word(&mut tokens, &text[start..end], start, include_parts);
        }
    }
    tokens
}

fn emit_word(tokens: &mut Vec<Token>, word: &str, offset: usize, include_parts: bool) {
    tokens.push(Token {
        term: word.chars().flat_map(char::to_lowercase).collect(),
        start: offset,
        end: offset + word.len(),
        colocated: false,
    });
    if include_parts {
        emit_parts(tokens, word, offset);
    }
}
