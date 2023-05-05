use super::{expect_valid_tokenization, Token};

#[test]
fn pub_inputs_kw() {
    let source = "public_inputs";
    let tokens = vec![Token::PublicInputs];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn pub_inputs_sized_arrays() {
    let source = "
public_inputs:
    program_hash: [4]
    stack_inputs: [12]";

    let tokens = vec![
        Token::PublicInputs,
        Token::Colon,
        Token::Ident("program_hash".to_string()),
        Token::Colon,
        Token::LBracket,
        Token::Num(4),
        Token::RBracket,
        Token::Ident("stack_inputs".to_string()),
        Token::Colon,
        Token::LBracket,
        Token::Num(12),
        Token::RBracket,
    ];
    expect_valid_tokenization(source, tokens);
}
