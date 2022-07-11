use mini_evm_calculator::{calculate, utils::errors::LexicalError, Uint};

#[test]
fn test_mini_evm_calculator() {
    let source = "156 + 4 * 3 / 2";
    assert_eq!(calculate(source), Ok(Uint::from_dec_str("162").unwrap()));
    let source = "(156 + 4) * 3 / 2";
    assert_eq!(calculate(source), Ok(Uint::from_dec_str("240").unwrap()));
    let source = "(156 + 4) * 3 ^ 2";
    assert_eq!(calculate(source), Err(LexicalError::InvalidCharacter('^')));
}
