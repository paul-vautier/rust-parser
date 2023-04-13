use crate::parser::json::json_value;

mod parser;
fn main() {
    println!(
        "{:?}",
        json_value("[true,      false, [false], \" \\\"abcdef\"]")
    );
}
