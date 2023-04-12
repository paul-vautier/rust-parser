pub type ParseError<'a> = (u32, &'a str);
pub type ParseResult<I, O, E> = Result<(I, O), E>;

pub trait Parser<'a, I: 'a, ParseError>
where
    I: Clone,
{
    type Output;

    fn and<G>(self, parser: G) -> And<Self, G>
    where
        G: Parser<'a, I, ParseError>,
        Self: Sized,
    {
        And {
            first: self,
            second: parser,
        }
    }

    fn or<G>(self, parser: G) -> Or<Self, G>
    where
        G: Parser<'a, I, ParseError>,
        Self: Sized,
    {
        Or {
            first: self,
            second: parser,
        }
    }

    fn map<F, O>(self, f: F) -> Map<F, Self>
    where
        F: Fn(Self::Output) -> O,
        Self: Sized,
    {
        Map { f, parser: self }
    }

    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many { parser: self }
    }

    fn parse(&mut self, input: I) -> ParseResult<I, Self::Output, ParseError>;
}

pub fn sep_by<'a, I: 'a, O: 'a, P, S>(parser: P, separator: S) -> Sep<P, S>
where
    P: Parser<'a, I, ParseError<'a>, Output = O>,
    S: Parser<'a, I, ParseError<'a>>,
    I: Clone,
{
    Sep { parser, separator }
}

pub fn wrapped<'a, I: 'a, O: 'a, L, P, R>(left: L, parser: P, right: R) -> Wrap<L, P, R>
where
    L: Parser<'a, I, ParseError<'a>>,
    P: Parser<'a, I, ParseError<'a>, Output = O>,
    R: Parser<'a, I, ParseError<'a>>,
    I: Clone,
{
    Wrap {
        left,
        parser,
        right,
    }
}

pub fn discard<'a, I: 'a, O: 'a, D, P>(discard: D, parser: P) -> Discard<D, P>
where
    P: Parser<'a, I, ParseError<'a>, Output = O>,
    D: Parser<'a, I, ParseError<'a>>,
    I: Clone,
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

pub struct Wrap<L, P, R> {
    pub(crate) left: L,
    pub(crate) parser: P,
    pub(crate) right: R,
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
