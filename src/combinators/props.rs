use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space1,
    multi::many0,
    sequence::tuple,
    IResult,
};

use serde::{Deserialize, Serialize};

use super::events::{event, EventProp};

#[derive(Debug, Serialize, Deserialize)]
pub enum Prop {
    Event(EventProp),
    Data(DataProp),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProp {
    key: String,
    value: String,
}

pub fn props0(input: &str) -> IResult<&str, Vec<Prop>> {
    // multiple event or data prop
    let (input, props) = many0(alt((data_prop, event)))(input)?;

    Ok((input, props))
}

fn data_prop(input: &str) -> IResult<&str, Prop> {
    let (input, _) = space1(input)?;
    let (input, (key, _, value)) = tuple((
        take_while1(|c: char| c.is_alphanumeric() || c == '-'),
        tag(":"),
        take_while1(|c: char| c.is_alphanumeric() || c == '.'),
    ))(input)?;

    Ok((
        input,
        Prop::Data(DataProp {
            key: key.to_string(),
            value: value.to_string(),
        }),
    ))
}
