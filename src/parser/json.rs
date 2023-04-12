use std::collections::HashMap;

use super::{
    impls::{opt, sequence, ws},
    traits::{sep_by, wrapped, ParseError, ParseResult, Parser},
};

#[derive(Debug)]
pub enum JsonValue {
    Array(Vec<JsonValue>),
    Boolean(bool),
    String(String),
    Number(f64),
    Object(HashMap<String, JsonValue>),
    Null,
}

pub fn null<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue, ParseError<'a>> {
    sequence("null").map(|_| JsonValue::Null).parse(input)
}

pub fn json_value<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue, ParseError<'a>> {
    wrapped(ws(), null.or(boolean).or(array), ws())(input)
}

pub fn array<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue, ParseError<'a>> {
    wrapped(
        sequence("["),
        sep_by(json_value, sequence(",")).map(JsonValue::Array),
        sequence("]"),
    )(input)
}

pub fn boolean<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue, ParseError<'a>> {
    sequence("true")
        .or(sequence("false"))
        .map(|str_bool| JsonValue::Boolean(str_bool == "true"))
        .parse(input)
}
