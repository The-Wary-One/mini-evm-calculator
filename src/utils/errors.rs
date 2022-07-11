use std::fmt::Display;

use crate::lexer::TokenList;

/// A lexical error
#[derive(Debug, PartialEq, Eq)]
pub enum LexicalError {
    /// Invalid characters
    InvalidCharacter(char),
    /// Invalid number
    InvalidNumber(String),
    /// Mismatched parenthesis
    MismatchedParenthesis,
    /// Invalid token list
    InvalidTokenList(TokenList),
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match &self {
            &LexicalError::InvalidCharacter(c) => format!("Invalid character: {}", c),
            &LexicalError::InvalidNumber(n) => format!("Invalid number: {}", n),
            &LexicalError::MismatchedParenthesis => String::from("Mismatched parenthesis"),
            &LexicalError::InvalidTokenList(t) => format!("Invalid source: {}", t),
        };
        write!(f, "{}", e)
    }
}
