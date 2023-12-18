use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Style {
    key: String,
    value: String,
}

/// Parses a string input and returns a vector of `Style` structs representing the style properties.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `nom` `IResult` containing the remaining input and a vector of `Style` structs.
///
/// # Examples
///
/// ```
/// use nom::IResult;
///
/// let input = "[color:red5, bg:blue0]";
/// let result: IResult<&str, Vec<Style>> = style_prop0(input);
///
/// assert_eq!(result.is_ok(), true);
/// let (remaining_input, styles) = result.unwrap();
/// assert_eq!(remaining_input, "");
/// assert_eq!(styles.len(), 2);
/// assert_eq!(styles[0].key, "color");
/// assert_eq!(styles[0].value, "red5");
/// assert_eq!(styles[1].key, "bg");
/// assert_eq!(styles[1].value, "blue0");
/// ```
pub fn style_props0(input: &str) -> IResult<&str, Vec<Style>> {
    let (input, tag_styles) = opt(style_list)(input)?;

    let styles: Vec<Style> = tag_styles
        .unwrap_or_default()
        .into_iter()
        .map(|(key, value)| Style {
            key: key.to_string(),
            value: value.to_string(),
        })
        .collect();

    Ok((input, styles))
}

fn style_list(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, tag_styles) =
        delimited(char('['), separated_list0(char(','), style), char(']'))(input)?;

    Ok((input, tag_styles))
}

fn style(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (key, _, value)) = tuple((
        take_while1(|c: char| c.is_alphanumeric() || c == '-'),
        tag(":"),
        take_while1(|c: char| c.is_alphanumeric() || c == '-'),
    ))(input)?;

    Ok((input, (key, value)))
}
