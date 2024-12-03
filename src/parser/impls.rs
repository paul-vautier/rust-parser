use std::{cmp, process::Output};

use super::{
    errors::{ErrorSource, ParserError},
    traits::{opt, And, Discard, DropUntil, Input, Many, Map, Or, ParseResult, Parser, Sep},
};

impl<I, P, D, O> Parser<I> for Discard<D, P>
where
    P: Parser<I, Output = O>,
    D: Parser<I>,
    I: Input,
{
    type Output = O;

    fn parse(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let (i, _) = self.discard.parse(input)?;
        self.parser.parse(i)
    }
}

impl<I, O1, O2, F, P> Parser<I> for Map<F, P>
where
    F: FnMut(O1) -> O2,
    P: Parser<I, Output = O1>,
    I: Input,
{
    type Output = O2;
    fn parse(&mut self, input: I) -> ParseResult<I, O2> {
        self.parser.parse(input).map(|(i, res)| (i, (self.f)(res)))
    }
}

impl<I, O, F> Parser<I> for F
where
    F: FnMut(I) -> ParseResult<I, O>,
    I: Input,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O> {
        self(input)
    }
}

impl<I, O> Parser<I> for Box<dyn Parser<I, Output = O>>
where
    I: Input,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O> {
        (**self).parse(input)
    }
}

impl<I, P> Parser<I> for Many<P>
where
    P: Parser<I>,
    I: Input,
{
    type Output = Vec<P::Output>;
    fn parse(&mut self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let mut parsed: Vec<P::Output> = vec![];
        let mut ipt = input;
        loop {
            if ipt.input_len() == 0 {
                break;
            }
            match self.parser.parse(ipt.clone()) {
                Ok((i, res)) => {
                    if i.input_len() == ipt.input_len() {
                        break;
                    }
                    ipt = i;
                    parsed.push(res);
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok((ipt, parsed))
    }
}

impl<I, P, S> Parser<I> for Sep<P, S>
where
    P: Parser<I>,
    S: Parser<I>,
    I: Input,
{
    type Output = Vec<P::Output>;
    fn parse(&mut self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let mut ans: Vec<P::Output> = vec![];
        let mut i = input;
        loop {
            if let Ok((next, res)) = self.parser.parse(i.clone()) {
                ans.push(res);
                i = next;
            } else {
                break;
            }
            if let Ok((next, _)) = self.separator.parse(i.clone()) {
                i = next;
            } else {
                break;
            }
        }
        Ok((i, ans))
    }
}

impl<I, F, S> Parser<I> for And<F, S>
where
    F: Parser<I>,
    S: Parser<I>,
    I: Input,
{
    type Output = (F::Output, S::Output);
    fn parse(&mut self, input: I) -> ParseResult<I, (F::Output, S::Output)> {
        let (input, first) = self.first.parse(input)?;
        let (input, second) = self.second.parse(input)?;
        return Ok((input, (first, second)));
    }
}

impl<I, O, F, S> Parser<I> for Or<F, S>
where
    F: Parser<I, Output = O>,
    S: Parser<I, Output = O>,
    I: Input,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O> {
        self.first.parse(input.clone()).or_else(|_| {
            return self.second.parse(input);
        })
    }
}
impl<I, S> Parser<I> for DropUntil<S>
where
    S: Parser<I>,
    I: Input,
{
    type Output = S::Output;
    fn parse(&mut self, input: I) -> ParseResult<I, S::Output> {
        let mut offset = 0;
        loop {
            if input.input_len() <= offset {
                return Err(ParserError::new(
                    0,
                    ErrorSource::DropUntil,
                    "could not find any match for drop until",
                ));
            }
            match self.until.parse(input.drop(offset)) {
                Ok(res) => return Ok(res),
                Err(_) => offset += 1,
            }
        }
    }
}

pub fn sequence<'a>(matcher: &'a str) -> impl Parser<&'a str, Output = &'a str> {
    move |input: &'a str| {
        if input.is_empty() {
            return Err(ParserError::new(
                0,
                ErrorSource::Sequence(matcher),
                "empty sequence",
            ));
        }
        match input
            .chars()
            .zip(matcher.chars())
            .position(|(first, second)| first != second)
        {
            Some(position) => Err(ParserError::new(
                position,
                ErrorSource::Sequence(matcher),
                format!(
                    "could not parse sequence '{}'",
                    &input[position..cmp::min(position + 10, input.len())]
                )
                .as_str(),
            )),
            None => {
                let (parsed, remainder) = input.split_at(matcher.len());
                return Ok((remainder, parsed));
            }
        }
    }
}

pub fn take_while<'a, P>(mut predicate: P) -> impl Parser<&'a str, Output = &'a str>
where
    P: FnMut(char) -> bool,
{
    move |input: &'a str| {
        if input.is_empty() {
            return Err(ParserError::new(
                0,
                ErrorSource::TakeWhile,
                "empty sequence",
            ));
        }
        match input.chars().position(|c| !(predicate)(c)) {
            Some(position) => {
                if position == 0 {
                    return Err(ParserError::new(
                        0,
                        ErrorSource::TakeWhile,
                        format!("could not parse for char {}", &input[0..1]).as_str(),
                    ));
                }

                let (parsed, remainder) = input.split_at(position);
                return Ok((remainder, parsed));
            }
            None => {
                return Ok(("", input));
            }
        };
    }
}

pub fn none_of<'a>(chars: &'a str) -> impl Parser<&'a str, Output = &'a str> {
    take_while(|c| !chars.contains(c))
}

pub fn not<'a>(chr: char) -> impl Parser<&'a str, Output = &'a str> {
    take_while(move |c| chr != c)
}

pub fn any<'a>(chars: &'a str) -> impl Parser<&'a str, Output = &'a str> {
    take_while(|c| chars.contains(c))
}

pub fn ws<'a>() -> impl Parser<&'a str, Output = Option<&'a str>> {
    opt(take_while(char::is_whitespace))
}
