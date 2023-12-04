use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{space0, space1},
    combinator::opt,
    IResult,
};

pub struct Attribute {
    pub name: String,
    pub a_type: String,
    pub is_positional: bool,
}

fn is_attribute_positional(input: &str) -> IResult<&str, bool> {
    let (input, positional_result) = opt(tag("positional"))(input)?;

    let is_positional = positional_result.is_some();

    let (input, _) = space0(input)?;

    Ok((input, is_positional))
}

pub fn attribute_tag(input: &str) -> IResult<&str, Attribute, nom::error::Error<&str>> {
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
        Attribute {
            name: String::from(attr_name),
            a_type: String::from(attr_type),
            is_positional,
        },
    ))
}
