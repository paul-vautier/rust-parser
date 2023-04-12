use std::collections::HashMap;

use super::{
    impls::{any, none_of, sequence, ws},
    traits::{discard, opt, sep_by, wrapped, ParseResult, Parser},
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

pub fn null<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    sequence("null").map(|_| JsonValue::Null).parse(input)
}

fn escaped<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
    sequence("\\\\")
        .map(|_| "\\")
        .or(sequence("\\\"").map(|_| "\""))
        .or(sequence("\\n").map(|_| "\\n"))
        .or(sequence("\\t").map(|_| "\\t"))
        .or(sequence("\\r").map(|_| "\\r"))
        .or(sequence("\\/").map(|_| "\\/"))
        .or(sequence("\\f").map(|_| "\\f"))
        .or(sequence("\\b").map(|_| "\\b"))
        .parse(input)
}

pub fn string<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    wrapped(
        sequence("\""),
        none_of("\"\\")
            .or(escaped)
            .many()
            .map(|vec| vec.into_iter().collect::<String>())
            .map(JsonValue::String),
        sequence("\""),
    )
    .parse(input)
}

pub fn json_value<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    discard(opt(ws()), null.or(boolean).or(string).or(array)).parse(input)
}

pub fn array<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    wrapped(
        sequence("["),
        sep_by(json_value, sequence(",")).map(JsonValue::Array),
        sequence("]"),
    )(input)
}

pub fn boolean<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    sequence("true")
        .or(sequence("false"))
        .map(|str_bool| JsonValue::Boolean(str_bool == "true"))
        .parse(input)
}
