mod parser;

use crate::parser::{
    impls::sequence,
    json::{boolean, json_value, null},
    traits::{sep_by, wrapped, Parser},
};
fn main() {
    println!("{:?}", json_value("[true, false, [false]]"));
}
