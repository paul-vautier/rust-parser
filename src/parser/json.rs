use std::collections::HashMap;

use super::{
    impls::{none_of, sequence, take_while, ws},
    traits::{discard, opt, parse_if, sep_by, wrapped, ParseResult, Parser},
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

pub fn json_object<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    wrapped(
        sequence("{"),
        sep_by(json_pair, sequence(",")),
        sequence("}"),
    )
    .map(Vec::into_iter)
    .map(Iterator::collect::<HashMap<String, JsonValue>>)
    .map(JsonValue::Object)
    .parse(input)
}

pub fn json_pair<'a>(input: &'a str) -> ParseResult<&'a str, (String, JsonValue)> {
    wrapped(
        ws(),
        string
            .map(String::from)
            .and(discard(wrapped(ws(), sequence(":"), ws()), json_value)),
        ws(),
    )(input)
}

pub fn null<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    sequence("null").map(|_| JsonValue::Null).parse(input)
}

fn escaped<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
    sequence("\\\\")
        .map(|_| "\\")
        .or(sequence("\\\"").map(|_| "\""))
        .or(sequence("\\n").map(|_| "\n"))
        .or(sequence("\\t").map(|_| "\t"))
        .or(sequence("\\r").map(|_| "\r"))
        .or(sequence("\\/").map(|_| "/"))
        .or(sequence("\\f").map(|_| "\u{000C}"))
        .or(sequence("\\b").map(|_| "\u{0008}"))
        .parse(input)
}

pub fn string<'a>(input: &'a str) -> ParseResult<&'a str, String> {
    wrapped(
        sequence("\""),
        none_of("\"\\")
            .or(escaped)
            .many()
            .map(|vec| vec.into_iter().collect::<String>()),
        sequence("\""),
    )
    .parse(input)
}

pub fn json_value<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    discard(
        ws(),
        null.or(boolean)
            .or(array)
            .or(json_object)
            .or(string.map(JsonValue::String)),
    )
    .parse(input)
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

pub fn json_number<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    opt(sequence("-"))
        .map(|opt| if opt.is_some() { -1 } else { 1 })
        .and(
            sequence("0")
                .or(digits)
                .map(str::parse::<u32>)
                .map(Result::unwrap),
        )
        .and(
            parse_if(sequence("."), digits)
                .map(|opt| opt.map(str::parse::<u32>).map(Result::unwrap).unwrap_or(0)),
        )
        .map(|_| JsonValue::Null)
        .parse(input)
}

pub fn digits<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
    take_while(|c| c.is_digit(10)).parse(input)
}
