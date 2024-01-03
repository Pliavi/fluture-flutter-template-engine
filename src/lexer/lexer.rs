use super::{
    combinators::{
        space_indentation_combinators::{capture_indentation, skip_whitespace},
        take_ident::tokenize_ident,
        take_number::tokenize_number,
        take_while::take_while,
    },
    token_struct::{Token, TokenKind},
};

use anyhow::{bail, Result};
use std::{io::ErrorKind, str};

pub fn tokenize_single_token(input: &str) -> Result<(TokenKind, usize)> {
    let next = match input.chars().next() {
        Some(c) => c,
        _ => bail!(ErrorKind::UnexpectedEof),
    };

    let (token_got, length) = match next {
        '*' => (TokenKind::Asterisk, 1),
        '=' => (TokenKind::Equals, 1),
        '+' => (TokenKind::Plus, 1),
        '/' => (TokenKind::Slash, 1),
        '<' => (TokenKind::LessThan, 1),
        '>' => (TokenKind::GreaterThan, 1),
        '-' => (TokenKind::Minus, 1),
        ':' => (TokenKind::Colon, 1),
        '@' => (TokenKind::At, 1),
        '.' => (TokenKind::Dot, 1),
        ')' => (TokenKind::CloseParen, 1),
        ']' => (TokenKind::CloseSquare, 1),
        '(' => (TokenKind::OpenParen, 1),
        '[' => (TokenKind::OpenSquare, 1),
        ';' => (TokenKind::Semicolon, 1),
        '0'..='9' => tokenize_number(input)?,
        '"' => {
            let (got, len_read) = take_while(&input[1..], |ch| ch != '"')?;
            let token = TokenKind::QuotedString(got.to_string());
            (token, len_read + 2)
        }
        c @ '_' | c if c.is_alphabetic() => tokenize_ident(input)?,
        // c if c.is_whitespace() => (_, skip_whitespace(input)),
        '\n' => capture_indentation(input)?,
        _ => bail!(ErrorKind::InvalidData), // ErrorKind::UnknownCharacter(other)
    };

    Ok((token_got, length))
}

pub fn lex(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let mut row = 1;
    let mut col_start = 1;
    let mut col_end;
    let mut is_line_start = true;

    loop {
        if !is_line_start {
            let ws = skip_whitespace(remaining);
            col_start += ws;
            remaining = &remaining[ws..]
        } else {
            is_line_start = false;
        }

        // TODO: maybe check for any whitespace too?
        if remaining.is_empty() {
            break;
        }

        let (token, len_read) = tokenize_single_token(remaining)?;
        match token {
            TokenKind::Indentation(_) => {
                is_line_start = true;
                row += 1;
                col_start = 1;
                col_end = col_start + len_read;
            }
            _ => {
                col_end = col_start + len_read;
            }
        }

        tokens.push(Token::new(token, col_start, col_end, row));

        col_start = col_end;
        remaining = &remaining[len_read..];
    }

    Ok(tokens)
}
