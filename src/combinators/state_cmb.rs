use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{space0, space1},
    IResult,
};

pub struct State {
    pub name: String,
    pub s_type: String,
}

pub fn state_tag(input: &str) -> IResult<&str, State, nom::error::Error<&str>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("state")(input)?;
    let (input, _) = space1(input)?;
    let (input, state_type) = take_until1(" ")(input)?;
    let (input, _) = space1(input)?;

    let (input, state_name) = alt((take_until1("="), take_until1(";")))(input)?;
    if input.starts_with("=") {
        // TODO: parse starting expression
    }

    let (input, _) = space0(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        State {
            name: String::from(state_name),
            s_type: String::from(state_type),
        },
    ))
}
