use super::traits::Input;

#[derive(Debug, PartialEq)]
pub enum ErrorSource<E: Input> {
    Many,
    Sequence(E),
    TakeWhile,
    DropUntil,
}

#[derive(Debug, PartialEq)]
pub struct ParserError<E: Input> {
    pub index: usize,
    pub source: ErrorSource<E>,
    pub reason: String,
}

impl<E> ParserError<E>
where
    E: Input,
{
    pub fn new(index: usize, source: ErrorSource<E>, reason: &str) -> Self {
        ParserError {
            index,
            source,
            reason: reason.to_string(),
        }
    }

    pub fn from_error(error: ParserError<E>, index: usize) -> Self {
        ParserError {
            index: error.index + index,
            source: error.source,
            reason: error.reason,
        }
    }
}
