#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Token {
    pub(crate) term: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) colocated: bool,
}

mod parts;
mod tokenize;

pub(crate) use tokenize::{tokenize_document, tokenize_query};
