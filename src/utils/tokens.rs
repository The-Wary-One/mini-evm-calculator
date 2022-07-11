use std::fmt::Display;

use ethers_core::types::U256;

/// A single Token
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token {
    /// An open parenthesis
    OpenParen,
    /// A close parenthesis
    CloseParen,
    /// Addition
    Add,
    /// Substraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Number
    Num(U256),
    /// A Space
    Whitespace,
}

impl Token {
    /// Public function that returns an operator info
    pub fn operator(&self) -> Option<Operator> {
        match self {
            &Token::Add | &Token::Sub => Some(Operator {
                precedence: 0,
                associativity: Associativity::Left,
                arity: 2,
            }),
            &Token::Mul | &Token::Div => Some(Operator {
                precedence: 1,
                associativity: Associativity::Left,
                arity: 2,
            }),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Token::OpenParen => String::from("("),
            Token::CloseParen => String::from(")"),
            Token::Add => String::from("+"),
            Token::Sub => String::from("-"),
            Token::Mul => String::from("*"),
            Token::Div => String::from("/"),
            Token::Num(n) => n.to_string(),
            Token::Whitespace => String::new(),
        };
        write!(f, "{}", t)
    }
}

/// Type of operator associativity
#[derive(PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

/// An operator
pub struct Operator {
    pub precedence: u8,
    pub associativity: Associativity,
    pub arity: u8,
}

impl Operator {
    /// Public function that returns whether this operator has a lower precedence than another
    pub fn has_lower_precedence_than(&self, other: &Operator) -> bool {
        self.precedence < other.precedence
    }
}
