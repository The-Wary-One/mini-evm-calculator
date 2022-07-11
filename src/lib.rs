pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod utils;

pub use ethers_core::abi::Uint;
use utils::errors::LexicalError;

pub fn calculate(source: &str) -> Result<Uint, LexicalError> {
    let tokens = lexer::lexer(source)?;
    if cfg!(debug_assertions) {
        println!("Lexer (str to Tokens)> {:?}", tokens);
    }
    let pn = parser::parse(tokens)?;
    if cfg!(debug_assertions) {
        println!("Parser (Tokens to Prefix Notation)> {}", pn);
    }
    let bytecode = pn.into();
    if cfg!(debug_assertions) {
        println!("Compiler (PN to Bytecode)> {}", bytecode);
    }
    Ok(utils::evm::EVM::execute(&bytecode))
}
