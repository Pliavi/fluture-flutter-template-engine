use super::take_while::take_while;
use crate::{lexer::token_struct::TokenKind, lexer_test};
use anyhow::Result;

pub fn tokenize_number(input: &str) -> Result<(TokenKind, usize)> {
    let mut dot_seen = false;
    let (got, len_read) = take_while(input, |ch| match ch {
        c if c.is_digit(10) => true,
        c if c == '.' && !dot_seen => {
            dot_seen = true;
            true
        }
        _ => false,
    })?;

    let number: f64 = got.parse()?;
    let token = TokenKind::Number(number);

    Ok((token, len_read))
}

lexer_test!(tokenize_a_single_digit_integer, tokenize_number, "1" => 1.0);
lexer_test!(tokenize_a_longer_integer, tokenize_number, "1234567890" => 1234567890.0);
lexer_test!(tokenize_basic_decimal, tokenize_number, "12.3" => 12.3);
lexer_test!(tokenize_string_with_multiple_decimal_points, tokenize_number, "12.3.456" => 12.3);
lexer_test!(FAIL: cant_tokenize_a_string_as_a_decimal, tokenize_number, "asdfghj");
lexer_test!(tokenizing_decimal_stops_at_alpha, tokenize_number, "123.4asdfghj" => 123.4);
