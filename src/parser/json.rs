use std::collections::HashMap;

use super::{
    impls::{sequence, ws},
    traits::{sep_by, wrapped, ParseError, ParseResult, Parser},
};

pub enum JsonValue {
    Array(Vec<JsonValue>),
    Boolean(bool),
    String(String),
    Number(f64),
    Object(HashMap<String, JsonValue>),
    Null,
}
