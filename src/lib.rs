use std::ops::Deref;

pub use pb_ebnf_macros::ebnf;

pub trait Single: Iterator + Sized {
    // Returns the element if the iterator only contains one item.
    fn single(mut self) -> Option<Self::Item> {
        self.next()
            .and_then(move |i| if self.next().is_none() { Some(i) } else { None })
    }
}

impl<T> Single for T where Self: Iterator {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syntax<'a>(&'a [Rule<'a>]);

impl<'a> Syntax<'a> {
    pub const fn new(rules: &'a [Rule<'a>]) -> Self {
        Self(rules)
    }
}

impl<'a> Deref for Syntax<'a> {
    type Target = [Rule<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule<'a> {
    pub lhs: Symbol<'a>,
    pub rhs: DefinitionsList<'a>,
}

impl<'a> Rule<'a> {
    pub const fn new(lhs: Symbol<'a>, rhs: DefinitionsList<'a>) -> Self {
        Self { lhs, rhs }
    }

    pub fn kernel(&self) -> impl Iterator<Item = &Symbol<'a>> {
        self.rhs.kernel()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionsList<'a>(&'a [SingleDefinition<'a>]);
impl<'a> DefinitionsList<'a> {
    pub const fn new(defs: &'a [SingleDefinition<'a>]) -> Self {
        Self(defs)
    }

    pub fn kernel(&self) -> impl Iterator<Item = &Symbol<'a>> {
        self.iter().flat_map(SingleDefinition::kernel)
    }
}
impl<'a> Deref for DefinitionsList<'a> {
    type Target = [SingleDefinition<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleDefinition<'a>(&'a [Term<'a>]);
impl<'a> SingleDefinition<'a> {
    pub const fn new(terms: &'a [Term<'a>]) -> Self {
        Self(terms)
    }
}
impl<'a> SingleDefinition<'a> {
    pub fn kernel(&self) -> Option<&Symbol<'a>> {
        self.iter()
            .map(Term::as_single_symbol)
            .filter(Option::is_some)
            .single()
            .flatten()
    }
}
impl<'a> Deref for SingleDefinition<'a> {
    type Target = [Term<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Term<'a> {
    pub factor: Factor<'a>,
    pub exception: Option<Exception<'a>>,
}

impl<'a> Term<'a> {
    pub const fn new(factor: Factor<'a>, exception: Option<Exception<'a>>) -> Self {
        Self { factor, exception }
    }

    pub fn as_single_symbol(&self) -> Option<&Symbol<'a>> {
        self.exception
            .is_none()
            .then(|| self.factor.as_single_symbol())
            .flatten()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exception<'a>(Factor<'a>);
impl<'a> Exception<'a> {
    pub const fn new(factor: Factor<'a>) -> Self {
        Self(factor)
    }
}
impl<'a> Deref for Exception<'a> {
    type Target = Factor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Factor<'a> {
    pub primary: Primary<'a>,
    pub repetition: Option<u32>,
}
impl<'a> Factor<'a> {
    pub const fn new(primary: Primary<'a>, repetition: Option<u32>) -> Self {
        Self {
            primary,
            repetition,
        }
    }

    pub fn as_single_symbol(&self) -> Option<&Symbol<'a>> {
        self.repetition
            .is_none()
            .then(|| self.primary.as_symbol())
            .flatten()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primary<'a> {
    Optional(OptionalSequence<'a>),
    Repeated(RepeatedSequence<'a>),
    Grouped(GroupedSequence<'a>),
    Symbol(Symbol<'a>),
    Literal(Literal<'a>),
    Empty,
}

impl<'a> Primary<'a> {
    pub fn as_symbol(&self) -> Option<&Symbol<'a>> {
        if let Self::Symbol(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalSequence<'a>(DefinitionsList<'a>);
impl<'a> OptionalSequence<'a> {
    pub const fn new(defs: DefinitionsList<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedSequence<'a>(DefinitionsList<'a>);
impl<'a> RepeatedSequence<'a> {
    pub const fn new(defs: DefinitionsList<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedSequence<'a>(DefinitionsList<'a>);
impl<'a> GroupedSequence<'a> {
    pub const fn new(defs: DefinitionsList<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol<'a>(&'a str);
impl<'a> Symbol<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal<'a>(&'a str);
impl<'a> Literal<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}
