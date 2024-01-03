use super::take_while::take_while;
use crate::{lexer::token_struct::TokenKind, lexer_test};
use anyhow::{bail, Result};
use std::{io::ErrorKind, str};

pub fn tokenize_ident(input: &str) -> Result<(TokenKind, usize)> {
    match input.chars().next() {
        Some(ch) if ch.is_digit(10) => bail!("Identifiers cannot start with a digit"),
        None => bail!(ErrorKind::UnexpectedEof),
        _ => {}
    }

    let (got, len_read) = take_while(input, |ch| ch.is_alphanumeric() || ch == '_')?;

    let tok = TokenKind::Identifier(got.to_string());

    Ok((tok, len_read))
}

lexer_test!(tokenize_a_single_letter, tokenize_ident, "F" => "F");
lexer_test!(tokenize_an_identifer, tokenize_ident, "Foo" => "Foo");
lexer_test!(tokenize_ident_containing_an_underscore, tokenize_ident, "Foo_bar" => "Foo_bar");
lexer_test!(FAIL: tokenize_ident_cant_start_with_number, tokenize_ident, "7Foo_bar");
lexer_test!(FAIL: tokenize_ident_cant_start_with_dot, tokenize_ident, ".Foo_bar");
