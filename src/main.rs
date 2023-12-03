use std::{error::Error, io};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{line_ending, space0, space1},
    combinator::opt,
    IResult,
};

pub struct Attribute {
    name: String,
    atype: String,
}

pub struct State {
    name: String,
    atype: String,
}

pub struct Widget {
    name: String,
    positional_attributes: Vec<Attribute>,
    named_attributes: Vec<Attribute>,
    states: Vec<State>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./examples/simple.arrow";
    let source = std::fs::read_to_string(file_path)?;
    let res = parse(&source);

    match res {
        Ok((_, code)) => {
            println!("{}", code);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}

fn parse(source: &str) -> IResult<&str, String> {
    let (input, result) = program(source)?;

    Ok((input, result))
}

fn program(input: &str) -> IResult<&str, String> {
    let mut widget = Widget {
        name: String::from(""),
        positional_attributes: vec![],
        named_attributes: vec![],
        states: vec![],
    };

    let (input, _) = space_line_end0(input)?;
    let (input, _) = space_line_end0(input)?;

    let (input, _) = tag("widget")(input)?;
    let (input, _) = space1(input)?;

    let (input, widget_name) = take_until1(" ")(input)?;
    widget.name = String::from(widget_name);

    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;

    let (input, _) = space_line_end0(input)?;

    let mut input = input;
    while !input.starts_with("}") {
        let (remaining_input, (attribute, is_positional)) = attribute(input)?;
        if is_positional {
            widget.positional_attributes.push(attribute);
        } else {
            widget.named_attributes.push(attribute);
        }

        let (remaining, _) = space_line_end0(remaining_input)?;
        let (remaining, _) = space0(remaining)?;

        input = remaining;
    }

    let (_, _) = tag("}")(input)?;

    let (_, code) = generate_code(widget).unwrap();

    Ok(("", code))
}

fn generate_code(widget: Widget) -> IResult<(), String> {
    let mut code = String::from("class ");

    code.push_str(&widget.name);
    code.push_str(" extends StatelessWidget {\n");

    for attribute in widget.positional_attributes.iter() {
        code.push_str("    ");
        code.push_str(&attribute.atype);
        code.push_str(" ");
        code.push_str(&attribute.name);
        code.push_str(";\n");
    }

    for attribute in widget.named_attributes.iter() {
        code.push_str("    ");
        code.push_str(&attribute.atype);
        code.push_str(" ");
        code.push_str(&attribute.name);
        code.push_str(";\n");
    }

    for state in widget.states.iter() {
        code.push_str("    ");
        code.push_str(&state.atype);
        code.push_str(" ");
        code.push_str(&state.name);
        code.push_str(";\n");
    }

    code.push_str("}\n");

    Ok(((), code))
}

fn space_line_end0(input: &str) -> IResult<&str, &str> {
    alt((line_ending, space0))(input)
}

fn is_attribute_positional(input: &str) -> IResult<&str, bool> {
    let (input, positional_result) = opt(tag("positional"))(input)?;

    let is_positional = positional_result.is_some();

    let (input, _) = space0(input)?;

    Ok((input, is_positional))
}

fn attribute(input: &str) -> IResult<&str, (Attribute, bool), nom::error::Error<&str>> {
    let (input, _) = space0(input)?;

    let (input, is_positional) = is_attribute_positional(input)?;

    let (input, _) = tag("attribute")(input)?;
    let (input, _) = space1(input)?;
    let (input, attr_type) = take_until1(" ")(input)?;
    let (input, _) = space1(input)?;

    let (input, attr_name) = alt((take_until1("="), take_until1(";")))(input)?;
    if input.starts_with("=") {
        // TODO: parse const expression
    }
    let (input, _) = space0(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        (
            Attribute {
                name: String::from(attr_name),
                atype: String::from(attr_type),
            },
            is_positional,
        ),
    ))
}
