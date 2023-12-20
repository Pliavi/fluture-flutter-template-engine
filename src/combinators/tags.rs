use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, line_ending, space0, space1},
    combinator::opt,
    combinator::peek,
    multi::{many0, many0_count, many1},
    sequence::{pair, preceded},
    IResult,
};

use serde::{Deserialize, Serialize};

use super::{
    props::{props0, Prop},
    style::{style_props0, StyleProp},
};

// Represents a tag, e.g., "<Button[bg:yellow-100] @tap:controller.increment>"
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    name: String,
    styles: Vec<StyleProp>,
    props: Vec<Prop>,
    value: Option<String>,
    children: Vec<Tag>,
}

pub fn root_tag(input: &str) -> IResult<&str, Tag> {
    let (_, curr_indent) = count_indentation(input)?;

    let (input, gotten_tag) = only_tag(input)?;
    let (input, _) = space0(input)?;

    let (input, value) = opt(take_until("\n"))(input)?;
    let value = value.map(|s| s.to_string());

    let (input, _) = line_ending(input)?;

    let (_, next_indent) = count_indentation(input)?;

    let (input, children) = if next_indent >= curr_indent {
        let spaces = " ".repeat(next_indent);

        let (input, children) = peek(many0(preceded(space0, root_tag)))(input)?;
        let (input, _) = many0(preceded(space0, root_tag))(input)?;

        (input, children)
    } else {
        (input, vec![])
    };

    let gotten_tag = Tag {
        value,
        children,
        ..gotten_tag
    };

    Ok((input, gotten_tag))
}

fn only_tag(input: &str) -> IResult<&str, Tag> {
    let (input, _) = char('<')(input)?;

    let (input, name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    let name = name.to_string();

    let (input, styles) = style_props0(input)?;
    let (input, props) = props0(input)?;

    let (input, _) = char('>')(input)?;

    Ok((
        input,
        Tag {
            name,
            styles,
            props,
            value: None,
            children: vec![],
        },
    ))
}

fn count_indentation(input: &str) -> IResult<&str, usize> {
    let (input, count) = many0_count(char(' '))(input)?;

    Ok((input, count))
}
