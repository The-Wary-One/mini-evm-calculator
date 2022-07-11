use std::{fmt::Display, str::FromStr};

use ethers_core::abi::Uint;

use crate::{codegen::Bytecode, utils::stack::Stack};

/// EVM Opcodes
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Opcode {
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    PUSH32 = 0x7F,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", *self as u8)
    }
}

#[derive(Debug)]
pub enum OpcodeError {
    InvalidOpcode(String),
}

impl FromStr for Opcode {
    type Err = OpcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "01" => Ok(Opcode::ADD),
            "02" => Ok(Opcode::MUL),
            "03" => Ok(Opcode::SUB),
            "04" => Ok(Opcode::DIV),
            "7F" => Ok(Opcode::PUSH32),
            _ => Err(OpcodeError::InvalidOpcode(s.to_owned())),
        }
    }
}

pub struct EVM {
    pub stack: Stack<Uint>,
}

impl EVM {
    /// Execute some bytecode
    pub fn execute(bytecode: &Bytecode) -> Uint {
        let mut stack = Stack::new();
        let mut bytecode = &bytecode.to_string()[..];
        while !bytecode.is_empty() {
            let (op, tail) = bytecode.split_at(2);
            bytecode = tail;
            match op.parse().unwrap() {
                Opcode::PUSH32 => {
                    let (uint, rest) = tail.split_at(64);
                    bytecode = rest;
                    stack.push(Uint::from_str_radix(uint, 16).unwrap()).unwrap();
                }
                Opcode::ADD => {
                    let res = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(res).unwrap();
                }
                Opcode::MUL => {
                    let res = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(res).unwrap();
                }
                Opcode::SUB => {
                    let res = stack.pop().unwrap() - stack.pop().unwrap();
                    stack.push(res).unwrap();
                }
                Opcode::DIV => {
                    let res = stack.pop().unwrap() / stack.pop().unwrap();
                    stack.push(res).unwrap();
                }
            };
        }

        stack.top()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::tests::bytecode;

    #[test]
    fn test_evm() {
        // (add 156 (div (mul 4 3) 2))
        // PUSH 2 PUSH 3 PUSH 4 MUL DIV
        // PUSH 156 ADD
        let b = bytecode(
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
        assert_eq!(EVM::execute(&b), 162usize.into());
    }
}
