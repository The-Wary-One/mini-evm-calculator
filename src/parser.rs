use std::{collections::VecDeque, fmt::Display};

use crate::{
    lexer::TokenList,
    utils::{errors::LexicalError, tokens::Token},
};

/// Polish notation token list
#[derive(PartialEq, Eq, Debug)]
pub struct PNTokenList(VecDeque<Token>);

pub fn parse(tokens: TokenList) -> Result<PNTokenList, LexicalError> {
    let len = tokens.len();
    // For operators
    let mut stack = Vec::with_capacity(len);

    tokens
        .clone()
        .into_iter()
        // Start from the ending token
        .rev()
        // Loop over all the tokens and yield output in a queue
        .try_fold(VecDeque::with_capacity(len), |mut acc, t| match t {
            // Operands
            Token::Num(_) => {
                acc.push_back(t);
                Ok(acc)
            }
            _ if t.operator().is_some() => {
                // Safe unwrap here
                let o = t.operator().unwrap();
                while stack
                    .last()
                    .and_then(|t2: &Token| t2.operator())
                    .filter(|o2| o.has_lower_precedence_than(o2))
                    .is_some()
                {
                    // Safe unwrap here
                    acc.push_back(stack.pop().unwrap());
                }
                stack.push(t);
                Ok(acc)
            }
            Token::CloseParen => {
                stack.push(t);
                Ok(acc)
            }
            Token::OpenParen => {
                while stack.last().filter(|&l| l != &Token::CloseParen).is_some() {
                    // Safe unwrap here
                    acc.push_back(stack.pop().unwrap());
                }

                if stack.is_empty() {
                    return Err(LexicalError::MismatchedParenthesis);
                }
                // pop off '('
                // Safe unwrap here
                stack.pop().unwrap();
                Ok(acc)
            }
            _ => Ok(acc),
        })
        // empty the operator stack
        .and_then(move |mut v| {
            while stack.last().is_some() {
                // Safe unwrap here
                v.push_back(stack.pop().unwrap());
            }

            if stack.is_empty() {
                Ok(v.into_iter().rev().collect::<VecDeque<_>>())
            } else {
                Err(LexicalError::InvalidTokenList(tokens))
            }
        })
        .map(|q| q.into())
}

impl From<VecDeque<Token>> for PNTokenList {
    fn from(v: VecDeque<Token>) -> Self {
        PNTokenList(v)
    }
}

impl IntoIterator for PNTokenList {
    type Item = Token;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for PNTokenList {
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lexer;

    // For unit testing in other modules
    pub fn pn_token_list(v: VecDeque<Token>) -> PNTokenList {
        PNTokenList(v)
    }

    #[test]
    fn test_parser() {
        // 156 + 4 * 3 / 2
        let v = lexer::tests::token_list(vec![
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
        // + 156 / * 4 3 2
        let expected = vec![
            Token::Add,
            Token::Num(156u8.into()),
            Token::Div,
            Token::Mul,
            Token::Num(4u8.into()),
            Token::Num(3u8.into()),
            Token::Num(2u8.into()),
        ]
        .into();
        assert_eq!(parse(v), Ok(PNTokenList(expected)));

        // (156 + 4) * 3 / 2
        let v = lexer::tests::token_list(vec![
            Token::OpenParen,
            Token::Num(156u8.into()),
            Token::Whitespace,
            Token::Add,
            Token::Whitespace,
            Token::Num(4u8.into()),
            Token::CloseParen,
            Token::Whitespace,
            Token::Mul,
            Token::Whitespace,
            Token::Num(3u8.into()),
            Token::Whitespace,
            Token::Div,
            Token::Whitespace,
            Token::Num(2u8.into()),
        ]);
        // / * + 156 4 3 2
        let expected = vec![
            Token::Div,
            Token::Mul,
            Token::Add,
            Token::Num(156u8.into()),
            Token::Num(4u8.into()),
            Token::Num(3u8.into()),
            Token::Num(2u8.into()),
        ]
        .into();
        assert_eq!(parse(v), Ok(PNTokenList(expected)));
    }
}
