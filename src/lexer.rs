use std::{fmt::Display, iter::Peekable, ops::Index, slice::SliceIndex, str::Chars};

use ethers_core::abi::Uint;

use crate::utils::{errors::LexicalError, tokens::Token};

/// A valid list of tokens
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenList(Vec<Token>);

impl TokenList {
    /// Public function to get the list length
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for TokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl IntoIterator for TokenList {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Idx> Index<Idx> for TokenList
where
    Idx: SliceIndex<[Token]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

/// The lexer encapsulated in a struct.
pub struct Lexer<'a> {
    /// The source code as peekable chars.
    pub chars: Peekable<Chars<'a>>,
    /// The raw source code.
    pub source: &'a str,
}

/// Perform lexical analysis
pub fn lexer(source: &str) -> Result<TokenList, LexicalError> {
    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
    let lexer = Lexer {
        source,
        chars: source.chars().peekable(),
    };
    lexer
        .into_iter()
        .collect::<Result<Vec<Token>, LexicalError>>()
        .map(|v| v.into())
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();

        if next.is_none() {
            return None;
        }

        // Safe unwrap here
        let c = next.unwrap();
        Some(match c {
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '0'..='9' => {
                let mut s = c.to_string();
                while let Some(d) = self.chars.next_if(|d| d.is_numeric()) {
                    s.push(d);
                }
                Uint::from_dec_str(&s)
                    .map(Token::Num)
                    .map_err(move |_e| LexicalError::InvalidNumber(s))
            }
            _ if c.is_whitespace() => Ok(Token::Whitespace),
            _ => Err(LexicalError::InvalidCharacter(c)),
        })
    }
}

impl From<Vec<Token>> for TokenList {
    fn from(tokens: Vec<Token>) -> Self {
        TokenList(tokens)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // For unit testing in other modules
    pub fn token_list(v: Vec<Token>) -> TokenList {
        TokenList(v)
    }

    #[test]
    fn test_lexer() {
        let source = "156 + 4 * 3 / 2";
        let expected = TokenList(vec![
            Token::Num(156u8.into()),
            Token::Whitespace,
            Token::Add,
            Token::Whitespace,
            Token::Num(4u8.into()),
            Token::Whitespace,
            Token::Mul,
            Token::Whitespace,
            Token::Num(3u8.into()),
            Token::Whitespace,
            Token::Div,
            Token::Whitespace,
            Token::Num(2u8.into()),
        ]);
        assert_eq!(lexer(source), Ok(expected));
        assert_eq!(lexer("1e"), Err(LexicalError::InvalidCharacter('e')));
    }
}
