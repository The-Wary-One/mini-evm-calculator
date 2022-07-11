use std::fmt::Display;

use crate::{
    parser::PNTokenList,
    utils::{evm::Opcode, tokens::Token},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Bytecode(String);

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn generate_bytecode(token: &Token) -> String {
    match token {
        &Token::Num(u) => format!("{}{:064X}", Opcode::PUSH32, u),
        Token::Add => Opcode::ADD.to_string(),
        Token::Mul => Opcode::MUL.to_string(),
        Token::Sub => Opcode::SUB.to_string(),
        Token::Div => Opcode::DIV.to_string(),
        _ => unreachable!(),
    }
}

// 156 + 4 * 3 / 2
// + 156 / * 4 3 2   <-- PN
// (add 156 (div (mul 4 3) 2))
// PUSH1 2 PUSH1 3 PUSH 4 MUL DIV
// PUSH1 156 ADD
impl From<PNTokenList> for Bytecode {
    fn from(tokens: PNTokenList) -> Self {
        let bytecode = tokens
            .into_iter()
            .rev()
            .map(|t| generate_bytecode(&t))
            .collect();
        Bytecode(bytecode)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::parser::tests::pn_token_list;

    pub fn bytecode(s: String) -> Bytecode {
        Bytecode(s)
    }

    #[test]
    fn test_bytecode_conversion() {
        // 156 + 4 * 3 / 2
        // + 156 / * 4 3 2
        let pn = pn_token_list(
            vec![
                Token::Add,
                Token::Num(156u8.into()),
                Token::Div,
                Token::Mul,
                Token::Num(4u8.into()),
                Token::Num(3u8.into()),
                Token::Num(2u8.into()),
            ]
            .into(),
        );
        // (add 156 (div (mul 4 3) 2))
        // PUSH 2 PUSH 3 PUSH 4 MUL DIV
        // PUSH 156 ADD
        let expected = Bytecode(
            vec![
                "7F",                                                               // PUSH32
                "0000000000000000000000000000000000000000000000000000000000000002", // 2
                "7F",                                                               // PUSH32
                "0000000000000000000000000000000000000000000000000000000000000003", // 3
                "7F",                                                               // PUSH32
                "0000000000000000000000000000000000000000000000000000000000000004", // 4
                "02",                                                               // MUL
                "04",                                                               // DIV
                "7F",                                                               // PUSH32
                "000000000000000000000000000000000000000000000000000000000000009C", // 156
                "01",                                                               // ADD
            ]
            .join(""),
        );
        assert_eq!(Bytecode::from(pn), expected);
    }
}
