use nom::{
    bytes::complete::{take_until, take_while1},
    character::complete::{char, line_ending, space0, space1},
    combinator::opt,
    multi::{many0, many0_count},
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

pub fn root_tag(indent: usize, input: &str) -> IResult<&str, Tag> {
    // take tag
    let (input, _) = space0(input)?;
    let (input, tag) = only_tag(input)?;
    let (input, _) = space0(input)?;

    // take value of tag(for example, the text of a text tag)
    let (input, value) = opt(take_until("\n"))(input)?;
    let value = value.map(|s| s.to_string());

    let (input, _) = line_ending(input)?;

    // start to check for children
    let (input, curr_indent) = count_indentation(input)?;
    // if there are no children, return the tag

    if curr_indent == 0 || input.is_empty() || curr_indent <= indent {
        return Ok((input, Tag { value, ..tag }));
    }

    let tag = Tag {
        value,
        children,
        ..tag
    };

    Ok((input, tag))
}

// Parser for a tag, e.g., "<Button[bg:yellow-100] @tap:controller.increment>"
// fn tag_list_from_indent(indent: usize, input: &str) -> IResult<&str, Vec<Tag>> {
//     let (input, curr_indent) = count_indentation(input)?;

//     if curr_indent == indent || input.is_empty() {
//         return Ok((input, vec![]));
//     }

//     let (input, tag) = root_tag(input)?;

//     let (input, mut tags) = tag_list_from_indent(indent, input)?;

//     tags.insert(0, tag);

//     Ok((input, tags))
// }

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
