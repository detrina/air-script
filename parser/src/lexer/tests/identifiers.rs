use miden_diagnostics::SourceIndex;

use super::{expect_any_error, expect_error_at_location, expect_valid_tokenization};
use crate::lexer::{LexicalError, Token};

// IDENTIFIERS VALID TOKENIZATION
// ================================================================================================

#[test]
fn keywords_with_identifiers() {
    let source = "enf clk' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Quote,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num(1),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn keyword_and_identifier_without_space() {
    let source = "enfclk' = clkdef + 1";
    let tokens = vec![
        // enfclk' is considered as an identifier by logos
        Token::Ident("enfclk".to_string()),
        Token::Quote,
        Token::Equal,
        // clkdef is considered as an identifier by logos
        Token::Ident("clkdef".to_string()),
        Token::Plus,
        Token::Num(1),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn number_and_identier_without_space() {
    let source = "enf 1clk' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Num(1),
        Token::Ident("clk".to_string()),
        Token::Quote,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num(1),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn valid_tokenization_next_token() {
    let source = "enf clk'' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Quote,
        // This is a parsing error, not a scanning error.
        Token::Quote,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num(1),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn valid_tokenization_indexed_trace_access() {
    let source = "enf $main[0]' = $main[1] + $aux[0] + $aux[1]'";
    let tokens = vec![
        Token::Enf,
        Token::DeclIdentRef("$main".to_string()),
        Token::LBracket,
        Token::Num(0),
        Token::RBracket,
        Token::Quote,
        Token::Equal,
        Token::DeclIdentRef("$main".to_string()),
        Token::LBracket,
        Token::Num(1),
        Token::RBracket,
        Token::Plus,
        Token::DeclIdentRef("$aux".to_string()),
        Token::LBracket,
        Token::Num(0),
        Token::RBracket,
        Token::Plus,
        Token::DeclIdentRef("$aux".to_string()),
        Token::LBracket,
        Token::Num(1),
        Token::RBracket,
        Token::Quote,
    ];
    expect_valid_tokenization(source, tokens);
}

// SCAN ERRORS
// ================================================================================================

#[test]
fn error_identifier_with_invalid_characters() {
    let source = "enf clk@' = clk + 1";
    // "@" is not in the allowed characters.
    let expected = LexicalError::UnexpectedCharacter {
        start: SourceIndex::UNKNOWN,
        found: '@',
    };
    expect_error_at_location(source, expected, 0, 7);
}

#[test]
fn return_first_invalid_character_error() {
    use miden_diagnostics::ByteIndex;

    let source = "enf clk@' = clk@ + 1";
    // "@" is not in the allowed characters.
    let err = expect_any_error(source);
    match err {
        LexicalError::UnexpectedCharacter { start, found: '@' } => {
            let expected = SourceIndex::new(start.source_id(), ByteIndex(7));
            assert_eq!(start, expected);
        }
        err => panic!("unexpected lexical error in source: {:#?}", err),
    }
}
