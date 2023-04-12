use super::traits::{And, Discard, Many, Map, Or, ParseError, ParseResult, Parser, Sep, Wrap};

impl<'a, I: 'a, P, D, E, O> Parser<'a, I, E> for Discard<D, P>
where
    P: Parser<'a, I, E, Output = O>,
    D: Parser<'a, I, E>,
    I: Clone,
{
    type Output = O;

    fn parse(&mut self, input: I) -> ParseResult<I, Self::Output, E> {
        let (i, _) = self.discard.parse(input)?;
        self.parser.parse(i)
    }
}

impl<'a, I: 'a, O1, O2, E, F, P> Parser<'a, I, E> for Map<F, P>
where
    F: Fn(O1) -> O2,
    P: Parser<'a, I, E, Output = O1>,
    I: Clone,
{
    type Output = O2;
    fn parse(&mut self, input: I) -> ParseResult<I, O2, E> {
        self.parser.parse(input).map(|(i, res)| (i, (self.f)(res)))
    }
}

impl<'a, I: 'a, O: 'a, E, F> Parser<'a, I, E> for F
where
    F: Fn(I) -> ParseResult<I, O, E>,
    I: Clone,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O, E> {
        self(input)
    }
}

impl<'a, I: 'a, P> Parser<'a, I, ParseError<'a>> for Many<P>
where
    P: Parser<'a, I, ParseError<'a>>,
    I: Clone,
{
    type Output = Vec<P::Output>;
    fn parse(&mut self, input: I) -> ParseResult<I, Vec<P::Output>, ParseError<'a>> {
        let mut ans: Vec<P::Output> = vec![];
        let mut ipt = input;
        loop {
            if let Ok((i, res)) = self.parser.parse(ipt.clone()) {
                ipt = i;
                ans.push(res);
            } else {
                break;
            }
        }
        Ok((ipt, ans))
    }
}

impl<'a, I: 'a, P, S> Parser<'a, I, ParseError<'a>> for Sep<P, S>
where
    P: Parser<'a, I, ParseError<'a>>,
    S: Parser<'a, I, ParseError<'a>>,
    I: Clone,
{
    type Output = Vec<P::Output>;
    fn parse(&mut self, input: I) -> ParseResult<I, Vec<P::Output>, ParseError<'a>> {
        let mut ans: Vec<P::Output> = vec![];
        let mut i = input;
        let mut res;
        loop {
            (i, res) = self.parser.parse(i)?;
            ans.push(res);
            if let Ok((next, _)) = self.separator.parse(i.clone()) {
                i = next;
            } else {
                break;
            }
        }
        Ok((i, ans))
    }
}

impl<'a, I: 'a, E, L, P, R, O> Parser<'a, I, E> for Wrap<L, P, R>
where
    L: Parser<'a, I, E>,
    P: Parser<'a, I, E, Output = O>,
    R: Parser<'a, I, E>,
    I: Clone,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O, E> {
        let (input, _) = self.left.parse(input)?;
        let (input, res) = self.parser.parse(input)?;
        let (input, _) = self.right.parse(input)?;
        return Ok((input, res));
    }
}

impl<'a, I: 'a, E, F, S> Parser<'a, I, E> for And<F, S>
where
    F: Parser<'a, I, E>,
    S: Parser<'a, I, E>,
    I: Clone,
{
    type Output = (F::Output, S::Output);
    fn parse(&mut self, input: I) -> ParseResult<I, (F::Output, S::Output), E> {
        let (input, first) = self.first.parse(input)?;
        let (input, second) = self.second.parse(input)?;
        return Ok((input, (first, second)));
    }
}

impl<'a, I: 'a, O, E, F, S> Parser<'a, I, E> for Or<F, S>
where
    F: Parser<'a, I, E, Output = O>,
    S: Parser<'a, I, E, Output = O>,
    I: Clone,
{
    type Output = O;
    fn parse(&mut self, input: I) -> ParseResult<I, O, E> {
        self.first.parse(input.clone()).or_else(|_| {
            return self.second.parse(input);
        })
    }
}

pub fn sequence<'a>(matcher: &'a str) -> impl Parser<'a, &'a str, ParseError, Output = &'a str> {
    move |input: &'a str| {
        if input.is_empty() {
            return Err((0, "empty sequence"));
        }
        match input
            .chars()
            .zip(matcher.chars())
            .position(|(first, second)| first != second)
        {
            Some(position) => Err((position as u32, &input[..position])),
            None => {
                let (parsed, remainder) = input.split_at(matcher.len());
                return Ok((remainder, parsed));
            }
        }
    }
}

pub fn take_while<'a, P>(predicate: P) -> impl Parser<'a, &'a str, ParseError<'a>, Output = &'a str>
where
    P: Fn(char) -> bool,
{
    move |input: &'a str| {
        if input.is_empty() {
            return Err((0, "empty sequence"));
        }
        match input.chars().position(|c| (predicate)(c)) {
            Some(position) => {
                let (parsed, remainder) = input.split_at(position);
                return Ok((remainder, parsed));
            }
            None => {
                return Ok(("", input));
            }
        }
    }
}

pub fn ws<'a>() -> impl Parser<'a, &'a str, ParseError<'a>, Output = &'a str> {
    take_while(char::is_whitespace)
}
