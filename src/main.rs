mod parser;

use crate::parser::{
    impls::sequence,
    traits::{sep_by, wrapped, Parser},
};
fn main() {
    println!(
        "{:?}",
        sequence("test")
            .and(sequence("abc").or(sequence("def")))
            .parse("testdefd")
    );

    println!(
        "{:?}",
        sep_by(
            sequence("1").map(str::parse::<f32>).map(Result::unwrap),
            sequence(",")
        )
        .parse("1,1,1")
    );

    println!(
        "{:?}",
        wrapped(
            sequence("[").many().and(sequence(" ").many()),
            sequence("test"),
            sequence("]")
        )
        .parse("[[[[[    test]")
    );

    println!(
        "{:?}",
        wrapped(sequence("["), sequence("1"), sequence("]"))
            .map(str::parse::<f32>)
            .parse("[1]")
    );
}
