# Combinatory parser library

This library is a personal project inspired from [Tsoding's video on combinatory parsers](https://youtu.be/N9RUqGYuGfw). 

The base `Parser` trait has been modified after reading [Nom's library's implementation](https://github.com/rust-bakery/nom) of the `Parser` trait, since it was more comfortable to use and fairly more just (They used both generics and associated types, I only used generics).

## Example 

Implementation of a JSON parser using the library

```rust
use std::collections::HashMap;

use pepser::parser::{
    impls::{any, none_of, sequence, take_while, ws},
    traits::{discard, opt, parse_if, sep_by, value, wrapped, ParseResult, Parser},
};

#[derive(Debug, PartialEq)]
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
        discard(ws(), sequence("}")),
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
            .or(string.map(JsonValue::String))
            .or(json_number),
    )
    .parse(input)
}

pub fn array<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    wrapped(
        sequence("["),
        wrapped(ws(), sep_by(json_value, sequence(",")), ws()).map(JsonValue::Array),
        sequence("]"),
    )(input)
}

pub fn boolean<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    sequence("true")
        .or(sequence("false"))
        .map(|str_bool| JsonValue::Boolean(str_bool == "true"))
        .parse(input)
}

#[rustfmt::skip]
pub fn json_number<'a>(input: &'a str) -> ParseResult<&'a str, JsonValue> {
    opt(sequence("-"))
        .map(|opt| if opt.is_some() { -1 } else { 1 })
        .and(integral_part)
        .and(decimal_part)
        .and(exponent)
        .map(|(((sign, integral), decimal), exponent)| JsonValue::Number(calculate_number(sign, integral, decimal, exponent)))
        .parse(input)
}

fn calculate_number(sign: i64, integral: u64, decimal: f64, exponent: i32) -> f64 {
    (sign as f64 * (integral as f64 + decimal)).powi(exponent)
}
#[rustfmt::skip]
fn integral_part<'a>(input: &'a str) -> ParseResult<&'a str, u64> {
    sequence("0")
                .or(digits)
                .map(str::parse::<u64>)
                .map(Result::unwrap).parse(input)
}

#[rustfmt::skip]
fn decimal_part<'a>(input: &'a str) -> ParseResult<&'a str, f64> {
    parse_if(sequence("."), digits).map(|opt| {
        opt.map(|double_str| format!("0.{}", double_str).parse::<f64>())
            .map(Result::unwrap)
            .unwrap_or(0.0)
    }).parse(input)
}

#[rustfmt::skip]
fn exponent<'a>(input: &'a str) -> ParseResult<&'a str, i32> {
    opt(discard(any("eE"), 
    opt(
            value(-1, sequence("-")).or(value(1 as i32, sequence("+")
            ))).map(|opt| opt.unwrap_or(1))
        ).and(digits).map(|(a, b)| a * b.parse::<i32>().unwrap())
    ).map(|opt| opt.unwrap_or(1))
    .parse(input)
}
pub fn digits<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
    take_while(|c| c.is_digit(10)).parse(input)
}

#[test]
fn parse_object() {
    use JsonValue::*;
    assert_eq!(
        Ok((
            "",
            Object(
                vec![
                    (
                        "description".to_string(),
                        String("the description of the test case".to_string())
                    ),
                    (
                        "schema".to_string(),
                        Object(
                            vec![(
                                "the schema that should".to_string(),
                                String("be validated against".to_string())
                            )]
                            .into_iter()
                            .collect()
                        )
                    ),
                    (
                        "tests".to_string(),
                        Array(vec![
                            Object(
                                vec![
                                    (
                                        "description".to_string(),
                                        String("a specific test of a valid instance".to_string())
                                    ),
                                    ("data".to_string(), String("the instance".to_string())),
                                    ("valid".to_string(), Boolean(true))
                                ]
                                .into_iter()
                                .collect()
                            ),
                            Object(
                                vec![
                                    (
                                        "description".to_string(),
                                        String(
                                            "another specific test this time, invalid".to_string()
                                        )
                                    ),
                                    ("data".to_string(), Number(-15.0)),
                                    ("valid".to_string(), Boolean(false))
                                ]
                                .into_iter()
                                .collect()
                            )
                        ])
                    )
                ]
                .into_iter()
                .collect()
            )
        )),
        json_value(
            "    {
            \"description\": \"the description of the test case\",
            \"schema\": {\"the schema that should\" : \"be validated against\"},
            \"tests\": [
                {
                    \"description\": \"a specific test of a valid instance\",
                    \"data\": \"the instance\",
                    \"valid\": true
                },
                {
                    \"description\": \"another specific test this time, invalid\",
                    \"data\": -15,
                    \"valid\": false
                }
            ]
        }"
        )
    );
}

```

## Use case 

This library is not really meant to be used by someone else than me. It is only a fun experiment to discover combinatory parsers, and it will be slowly be completed with time as it will be the core parser for my own programming language.

Right now, it only supports string parsing, but I may implement ways to parse other streams.