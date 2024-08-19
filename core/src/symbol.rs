use crate::{LiteralRef, MetaIdentifierRef, OwnedLiteral, OwnedMetaIdentifier};
use std::ops::Deref;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct SymbolRef<'a>(&'a str);

pub type StaticSymbol = SymbolRef<'static>;

impl<'a> SymbolRef<'a> {
    pub fn to_owned(&self) -> OwnedSymbol {
        OwnedSymbol(self.0.to_owned())
    }
}

impl<'a> SymbolRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}

impl Deref for SymbolRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a str> for SymbolRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a OwnedLiteral> for SymbolRef<'a> {
    fn from(value: &'a OwnedLiteral) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a OwnedMetaIdentifier> for SymbolRef<'a> {
    fn from(value: &'a OwnedMetaIdentifier) -> Self {
        Self(value)
    }
}

impl<'a> From<LiteralRef<'a>> for SymbolRef<'a> {
    fn from(value: LiteralRef<'a>) -> Self {
        Self(value.0)
    }
}

impl<'a> From<MetaIdentifierRef<'a>> for SymbolRef<'a> {
    fn from(value: MetaIdentifierRef<'a>) -> Self {
        Self(value.0)
    }
}

pub enum SymbolIterator<'a> {
    Slice(std::slice::Iter<'a, SymbolRef<'a>>),
    Once(std::iter::Once<SymbolRef<'a>>),
    Empty(std::iter::Empty<SymbolRef<'a>>),
    Boxed(Box<dyn Iterator<Item = SymbolRef<'a>> + 'a>),
}

impl<'a> Iterator for SymbolIterator<'a> {
    type Item = SymbolRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SymbolIterator::Slice(iter) => iter.copied().next(),
            SymbolIterator::Once(iter) => iter.next(),
            SymbolIterator::Empty(iter) => iter.next(),
            SymbolIterator::Boxed(iter) => iter.next(),
        }
    }
}

impl<'a> SymbolIterator<'a> {
    pub fn boxed<T>(iter: T) -> Self
    where
        T: Iterator<Item = SymbolRef<'a>> + 'a,
    {
        Self::Boxed(Box::new(iter))
    }
}

impl<'a> From<std::slice::Iter<'a, SymbolRef<'a>>> for SymbolIterator<'a> {
    fn from(value: std::slice::Iter<'a, SymbolRef<'a>>) -> Self {
        Self::Slice(value)
    }
}

impl<'a> From<std::iter::Once<SymbolRef<'a>>> for SymbolIterator<'a> {
    fn from(value: std::iter::Once<SymbolRef<'a>>) -> Self {
        Self::Once(value)
    }
}

impl<'a> From<std::iter::Empty<SymbolRef<'a>>> for SymbolIterator<'a> {
    fn from(value: std::iter::Empty<SymbolRef<'a>>) -> Self {
        Self::Empty(value)
    }
}

pub trait BoxableSymbolIterator<'a>: Iterator<Item = SymbolRef<'a>> + Sized + 'a {
    fn into_boxed_iterator(self) -> SymbolIterator<'a> {
        SymbolIterator::boxed(self)
    }
}

impl<'a, T> BoxableSymbolIterator<'a> for T where T: Iterator<Item = SymbolRef<'a>> + Sized + 'a {}

pub trait SymbolIterable<'a>: Sized {
    fn iter_symbols(self) -> SymbolIterator<'a>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedSymbol(String);
