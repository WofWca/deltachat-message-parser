///! Base utility parsers, used by both text and markdown parsers
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    NoContent,
    InvalidWhiteSpaceFound,
    NoElement,
    Nom(I, ErrorKind),
    InvalidEmail,
    InvalidLink,
    UnexpectedContent,
    PrecedingWhitespaceMissing,
}

impl<I> ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub(crate) fn is_white_space(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\t' | ' ')
}

pub(crate) fn is_not_white_space(c: char) -> bool {
    !is_white_space(c)
}

pub(crate) fn is_white_space_but_not_linebreak(c: char) -> bool {
    matches!(c, '\t' | ' ')
}

/// delimited no whitespace start or end
pub(crate) fn direct_delimited<'a>(
    input: &'a str,
    tag_str: &str,
) -> IResult<&'a str, &'a str, CustomError<&'a str>> {
    let (input, content): (&str, &str) = delimited(
        tag(tag_str),
        nom::bytes::complete::is_not(tag_str),
        tag(tag_str),
    )(input)?;
    if content.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    if is_white_space(content.chars().next().unwrap())
        || is_white_space(content.chars().last().unwrap())
    {
        return Err(nom::Err::Error(CustomError::InvalidWhiteSpaceFound));
    }
    Ok((input, content))
}
