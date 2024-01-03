use crate::lexer::token_struct::{Token as Lexeme, TokenKind};
use anyhow::Result;
use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    MissingToken,
    InvalidExpression,
}

enum Token {
    Widget,
}

#[derive(Debug)]
pub struct Widget {
    name: String,
    children: Vec<Widget>,
}

pub fn parse_program(tokens: &mut VecDeque<Lexeme>) -> Result<Vec<Widget>, ParseError> {
    let mut result = Vec::new();

    let tk = tokens.front().unwrap();
    println!("tk: {:?}", tk);

    while let Some(lexeme) = tokens.front() {
        match lexeme.kind {
            TokenKind::WidgetKW => {}
            TokenKind::LessThan => {
                let widget = parse_widget(tokens, 0)?;
                result.push(widget);
            }
            TokenKind::Indentation(indent) => {
                tokens.pop_front();
                let widget = parse_widget(tokens, indent)?;
                result.push(widget);
            }
            _ => {
                return Err(ParseError::UnexpectedToken(format!(
                    "parse_program {:?}",
                    lexeme
                )))
            }
        }
    }
    Ok(result)
}

fn parse_children(
    tokens: &mut VecDeque<Lexeme>,
    indentation: usize,
) -> Result<Vec<Widget>, ParseError> {
    let mut result = Vec::new();

    while let Some(lexeme) = tokens.front() {
        match lexeme.kind {
            TokenKind::Indentation(indent) => {
                tokens.pop_front();
                let widget = parse_widget(tokens, indent)?;
                result.push(widget);
            }
            _ => {
                return Err(ParseError::UnexpectedToken(format!(
                    "parse_children {:?}",
                    lexeme
                )))
            }
        }
    }

    Ok(result)
}

fn parse_widget(tokens: &mut VecDeque<Lexeme>, indentation: usize) -> Result<Widget, ParseError> {
    let mut name: Option<String> = Some(String::from("None"));
    let mut widget = None;

    while let Some(lexeme) = tokens.front() {
        let lexeme_kind = lexeme.kind.clone();

        match lexeme_kind {
            TokenKind::LessThan => {
                tokens.pop_front();
            }
            TokenKind::GreaterThan => {
                let name_clone = name.clone().unwrap();
                if name_clone != "" {
                    let name = name_clone;

                    let mut children = Vec::new();

                    while let Some(lexeme) = tokens.front() {
                        match lexeme.kind {
                            TokenKind::Indentation(indent) => {
                                tokens.pop_front();
                                if indent > indentation {
                                    children
                                        .append(&mut parse_children(tokens, indentation).unwrap());
                                } else {
                                    break;
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    widget = Some(Widget { name, children });
                } else {
                    return Err(ParseError::InvalidExpression);
                }
            }
            TokenKind::Identifier(id) => {
                tokens.pop_front();
                name = Some(id);
            }
            TokenKind::Indentation(indent) => {
                break;
            }
            _ => {
                return Err(ParseError::UnexpectedToken(format!(
                    "parse_widget {:?}",
                    lexeme
                )))
            }
        }
    }

    return Ok(widget.unwrap_or(Widget {
        name: String::from("None"),
        children: vec![],
    }));
}
