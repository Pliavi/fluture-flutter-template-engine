use super::take_while::take_while;
use crate::lexer::token_struct::TokenKind;
use anyhow::Result;

pub fn skip_whitespace(input: &str) -> usize {
    let first_char = match input.chars().next() {
        Some(ch) => ch,
        _ => return 0,
    };

    if !first_char.is_ws_without_nl() {
        return 0;
    }

    match take_while(input, |ch| ch != '\n' && ch.is_whitespace()) {
        Ok((_, len_skipped)) => len_skipped,
        _ => 0,
    }
}

pub fn capture_indentation(input: &str) -> Result<(TokenKind, usize)> {
    let length = match take_while(input, |ch| ch.is_whitespace()) {
        Ok((_, len_skipped)) => len_skipped,
        _ => 0,
    };

    Ok((TokenKind::Indentation(length), length))
}

trait CharExtension {
    fn is_ws_without_nl(&self) -> bool;
}

impl CharExtension for char {
    fn is_ws_without_nl(&self) -> bool {
        self.is_whitespace() && *self != '\n'
    }
}

#[test]
fn skip_past_several_whitespace_chars() {
    let src = " \t\n\r123";
    let should_be = 4;

    let num_skipped = skip_whitespace(src);
    assert_eq!(num_skipped, should_be);
}

#[test]
fn skipping_whitespace_when_first_is_a_letter_returns_zero() {
    let src = "Hello World";
    let should_be = 0;

    let num_skipped = skip_whitespace(src);
    assert_eq!(num_skipped, should_be);
}
