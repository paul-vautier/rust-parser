use super::{errors::ParserError, impls::sequence};

pub type ParseResult<I, O> = Result<(I, O), ParserError<I>>;

pub trait Input: Clone {
    fn to_string_value(&self) -> String;

    fn input_len(&self) -> usize;
}

impl Input for &str {
    fn to_string_value(&self) -> String {
        self.to_string()
    }

    fn input_len(&self) -> usize {
        self.len()
    }
}

/// Combinatory parser trait
/// All parsers must implement this trait
pub trait Parser<I: Input> {
    type Output;

    /// Chains two parsers to return their output in a tuple
    ///  
    /// # Examples
    /// ```rust
    ///
    /// use super::impls::sequence;
    /// let mut parser = sequence("abc").and("def");
    ///
    /// assert_eq!(parser.parse("abcdefg"), Ok(("g", ("abc", "def"))));
    /// assert_eq!(parser.parse("abcdef"), Ok(("", ("abc", "def"))));
    /// assert_eq!(parser.parse("").is_err(), true);
    /// assert_eq!(parser.parse("defabc").is_err(), true);
    ///
    ///
    /// ```
    fn and<G>(self, parser: G) -> And<Self, G>
    where
        G: Parser<I>,
        Self: Sized,
    {
        And {
            first: self,
            second: parser,
        }
    }

    /// Chains a second parser to be tested if the first one fails.
    /// Returns an error if both parsers fail
    ///  
    /// # Examples
    /// ```rust
    ///
    /// use super::impls::sequence;
    /// let mut parser = sequence("abc").or("def");
    ///
    /// assert_eq!(parser.parse("abcdef"), Ok(("def", "abc")));
    /// assert_eq!(parser.parse("defabc"), Ok(("abc", "def")));
    /// assert_eq!(parser.parse("").is_err(), true);
    /// assert_eq!(parser.parse("123").is_err(), true);
    ///
    ///
    /// ```
    fn or<G>(self, parser: G) -> Or<Self, G>
    where
        G: Parser<I>,
        Self: Sized,
    {
        Or {
            first: self,
            second: parser,
        }
    }

    /// Applies a function to be applied to the output of the parser
    ///  
    /// # Examples
    /// ```rust
    ///
    /// use super::impls::sequence;
    /// let mut parser = sequence("123").map(str::parse::<u32>).map(Result::unwrap()).map(|v| v * 2);
    ///
    /// assert_eq!(parser.parse("123"), Ok(("", 246)));
    ///
    ///
    /// ```
    fn map<F, O>(self, f: F) -> Map<F, Self>
    where
        F: Fn(Self::Output) -> O,
        Self: Sized,
    {
        Map { f, parser: self }
    }

    /// Retries a parser until it fails.
    /// Returns an error if the parser fails on the first time
    ///
    /// # Examples
    /// ```rust
    ///
    /// use super::impls::sequence;
    /// let mut parser = sequence("123").many();
    ///
    /// assert_eq!(parser.parse("123123123123"), Ok(("", "123123123123")));
    /// assert_eq!(parser.parse("123"), Ok(("", "123")));
    /// assert_eq!(parser.parse("1231234"), Ok(("4", "123123")));
    /// assert_eq!(parser.parse("").is_err(), true);
    ///
    ///
    /// ```
    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many { parser: self }
    }

    fn parse(&mut self, input: I) -> ParseResult<I, Self::Output>;
}

pub trait FlattenTuple {
    type Output;
    fn into_flattened(self) -> Self::Output;
}

impl<A, B, C> FlattenTuple for ((A, B), C) {
    type Output = (A, B, C);

    fn into_flattened(self) -> Self::Output {
        ((self.0).0, (self.0).1, self.1)
    }
}

pub fn parse_if<I, O, C, P>(
    mut cond: C,
    mut parser: P,
) -> impl FnMut(I) -> ParseResult<I, Option<O>>
where
    I: Input,
    C: Parser<I>,
    P: Parser<I, Output = O>,
{
    move |ipt| match cond.parse(ipt.clone()) {
        Ok((i, _)) => parser.parse(i).map(|(i, r)| (i, Some(r))),
        Err(_) => Ok((ipt, None)),
    }
}

pub fn sep_by<'a, I, O, P, S>(parser: P, separator: S) -> Sep<P, S>
where
    I: Input,
    P: Parser<I, Output = O>,
    S: Parser<I>,
{
    Sep { parser, separator }
}

pub fn wrapped<I, O, L, P, R>(
    mut left: L,
    mut parser: P,
    mut right: R,
) -> impl FnMut(I) -> ParseResult<I, O>
where
    L: Parser<I>,
    P: Parser<I, Output = O>,
    R: Parser<I>,
    I: Input,
{
    move |input: I| {
        let (input, _) = left.parse(input)?;
        let (input, res) = parser.parse(input)?;
        let (input, _) = right.parse(input)?;
        return Ok((input, res));
    }
}

pub fn opt<I, O, F>(mut f: F) -> impl FnMut(I) -> ParseResult<I, Option<O>>
where
    I: Input,
    F: Parser<I, Output = O>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(_) => Ok((i, None)),
        }
    }
}

pub fn value<V: Clone, I, O, F>(v: V, mut f: F) -> impl FnMut(I) -> ParseResult<I, V>
where
    I: Input,
    F: Parser<I, Output = O>,
{
    move |input: I| f.parse(input).map(|(i, _)| (i, v.clone()))
}

pub fn discard<'a, I: 'a, O: 'a, D, P>(discard: D, parser: P) -> Discard<D, P>
where
    P: Parser<I, Output = O>,
    D: Parser<I>,
    I: Input,
{
    Discard { discard, parser }
}

pub struct Many<P> {
    pub(crate) parser: P,
}

pub struct Sep<P, S> {
    pub(crate) parser: P,
    pub(crate) separator: S,
}

pub struct And<F, S> {
    pub(crate) first: F,
    pub(crate) second: S,
}

pub struct Or<F, S> {
    pub(crate) first: F,
    pub(crate) second: S,
}

pub struct Map<F, P> {
    pub(crate) f: F,
    pub(crate) parser: P,
}

pub struct Discard<D, P> {
    pub(crate) discard: D,
    pub(crate) parser: P,
}
