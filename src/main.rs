mod combinators;
use combinators::{
    events::{event, Event},
    style::{style_props0, Style},
};

use nom::{
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, line_ending, multispace1, space0, space1},
    combinator::opt,
    error::Error,
    multi::{many0, many0_count},
    sequence::{delimited, tuple},
    IResult,
};
// Represents a tag, e.g., "<Button[bg:yellow-100] @tap:controller.increment>"
#[derive(Debug)]
struct Tag {
    name: String,
    styles: Vec<Style>,
    // TODO: change to parameters(that can be any other parameters, except style)
    events: Vec<Event>,
    value: Option<String>,
    children: Vec<Tag>,
}

// Represents a parameter, e.g., "controller: CounterPageController"
struct Parameter {
    name: String,
    type_: String,
}

// Represents the widget structure
struct Widget {
    name: String,
    parameters: Vec<Parameter>,
    body: Vec<Tag>,
}

fn count_indentation(input: &str) -> IResult<&str, usize> {
    let (input, count) = many0_count(space0)(input)?;

    Ok((input, count))
}

fn tag_props(input: &str) -> IResult<&str, Tag> {
    let (input, _) = char('<')(input)?;
    let (input, tag_name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    let (input, tag_styles) = style_props0(input)?;

    // TODO: Use parameter parser to group events, and other parameters types
    //       And be able to ignore the order of the parameters
    let (input, tag_events) = many0(event)(input)?;

    let (input, _) = char('>')(input)?;

    Ok((
        input,
        Tag {
            name: tag_name.to_string(),
            styles: tag_styles,
            events: tag_events,
            value: None,
            children: vec![],
        },
    ))
}

// Parser for a tag, e.g., "<Button[bg:yellow-100] @tap:controller.increment>"
fn tag_list_from_indent(input: &str) -> IResult<&str, Tag> {
    let (input, tag) = tag_props(input)?;

    let (input, _) = space0(input)?;
    let (input, value) = opt(take_until("\n"))(input)?;

    let (input, _) = line_ending(input)?;

    let (input, curr_indent) = count_indentation(input)?;

    // FIX: not setting the children based on the indentation
    // let (input, children) = many0(|input| tag_list_from_indent(curr_indent, input))(input)?;

    let tag = Tag {
        value: value.map(|s| s.to_string()),
        children: vec![],
        ..tag
    };

    Ok((input, (tag)))
}

// A parser for a single parameter, e.g., "controller: CounterPageController"
fn parameter(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (name, _, _, _, type_)) = tuple((
        take_while(|c: char| c.is_alphanumeric()),
        space0,
        tag(":"),
        space0,
        take_while(|c: char| c.is_alphanumeric()),
    ))(input)?;

    Ok((input, (name, type_)))
}

// A parser for the parameters list, e.g., "(controller: CounterPageController)"
fn parameters(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    delimited(
        char('('),
        nom::multi::separated_list0(char(','), parameter),
        char(')'),
    )(input)
}

// The main parser for the whole syntax
fn widget_parser(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, &str)>, Vec<Tag>)> {
    let (input, _) = tag("widget")(input)?;
    let (input, _) = space1(input)?;
    let (input, widget_name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = space0(input)?;
    let (input, params) = parameters(input)?;
    let (input, _) = multispace1(input)?;
    let (input, tags) = many0(tag_list_from_indent)(input)?;

    Ok((input, ("widget", widget_name, params, tags)))
}

fn main() {
    let input = "\
      widget CounterPage(controller: CounterPageController)
        <Button[bg:yellow-100] @tap:controller.increment>
            <Text> \"Counter: \"
            <Text> controller.counter
  ";

    match widget_parser(input) {
        Ok((_, result)) => println!("Parsed successfully: {:?}", result),
        Err(e) => println!("Error parsing input: {:?}", e),
    }
}
