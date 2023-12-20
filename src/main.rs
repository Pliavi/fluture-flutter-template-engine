mod combinators;
use combinators::tags::{root_tag, Tag};

use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace1, space0, space1},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use serde::{Deserialize, Serialize};

// Represents a parameter, e.g., "controller: CounterPageController"
#[derive(Debug, Serialize, Deserialize)]
struct Parameter {
    name: String,
    type_: String,
}

// Represents the widget structure
#[derive(Debug, Serialize, Deserialize)]
struct Widget {
    name: String,
    parameters: Vec<Parameter>,
    body: Tag,
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
fn widget_parser(input: &str) -> IResult<&str, Widget> {
    let (input, _) = tag("widget")(input)?;
    let (input, _) = space1(input)?;
    let (input, widget_name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = space0(input)?;
    let (input, params) = parameters(input)?;
    let (input, _) = multispace1(input)?;
    let (input, body) = root_tag(input)?;

    Ok((
        input,
        Widget {
            name: widget_name.to_string(),
            parameters: params
                .iter()
                .map(|(name, type_)| Parameter {
                    name: name.to_string(),
                    type_: type_.to_string(),
                })
                .collect(),
            body,
        },
    ))
}

fn main() {
    let input = "\
widget CounterPage(controller: CounterPageController)
  <Button1[bg:yellow-100] @tap:controller.increment>
    <Text> \"Increment\"
  <Button2[bg:red-100] @tap:controller.decrement>
    <Text> \"Decrement\"
  <Container>
    <GlowingBox>
      <WavingAnimation> 
        <Text> Happy hacking!
    <FittedBox>
      <Text> \"Counter: \" + controller.counter
  ";

    match widget_parser(input) {
        Ok((_, result)) => {
            let result = serde_json::to_string(&result).unwrap();

            println!("Parsed successfully: \n{}", result);
        }
        Err(e) => println!("Error parsing input: {:?}", e),
    };
}
