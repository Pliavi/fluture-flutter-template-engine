use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1},
    sequence::tuple,
    IResult,
};

// Represents an event, e.g., "@tap:controller.increment"
#[derive(Debug)]
pub struct Event {
    key: String,
    function: String,
}

// Parser for an event, e.g., "@tap:controller.increment"
pub fn event(input: &str) -> IResult<&str, Event> {
    let (input, _) = space1(input)?;
    let (input, (_, key, _, function)) = tuple((
        char('@'),
        take_while1(|c: char| c.is_alphanumeric() || c == '-'),
        tag(":"),
        take_while1(|c: char| c.is_alphanumeric() || c == '.'),
    ))(input)?;

    Ok((
        input,
        Event {
            key: key.to_string(),
            function: function.to_string(),
        },
    ))
}
