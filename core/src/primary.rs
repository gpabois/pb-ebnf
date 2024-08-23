use std::iter::Once;

use pb_bnf::{prelude::IterSymbols, symbol::Symbol};

use crate::{
    grouped::{GroupedSequence, GroupedSequenceRef, IGroupedSequence},
    literal::{ILiteral, Literal, LiteralRef},
    meta_identifier::{IMetaIdentifier, MetaIdentifier, MetaIdentifierRef},
    optional::{IOptionalSequence, OptionalSequence, OptionalSequenceRef},
    repeated::{IRepeatedSequence, RepeatedSequence, RepeatedSequenceRef},
    IntoTerm, Term,
};

pub trait IntoPrimary {
    type Primary: IPrimary;

    fn into_primary(self) -> Self::Primary;
}

pub trait AsPrimaryRef {
    type Primary: IPrimary;

    fn as_primary_ref(&self) -> &Self::Primary;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryKind {
    OptionalSequence,
    RepeatedSequence,
    GroupedSequence,
    MetaId,
    Literal,
    Empty,
}

pub trait IPrimary {
    type OptionalSequence: IOptionalSequence;
    type RepeatedSequence: IRepeatedSequence;
    type GroupedSequence: IGroupedSequence;
    type MetaIdentifier: IMetaIdentifier;
    type Literal: ILiteral;

    fn try_as_optional(&self) -> Option<&Self::OptionalSequence>;
    fn try_as_repeated(&self) -> Option<&Self::RepeatedSequence>;
    fn try_as_grouped(&self) -> Option<&Self::GroupedSequence>;
    fn try_as_meta_identifier(&self) -> Option<&Self::MetaIdentifier>;
    fn try_as_literal(&self) -> Option<&Self::Literal>;

    fn kind(&self) -> PrimaryKind;

    fn to_owned(self) -> Primary;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryRef<'a> {
    Optional(OptionalSequenceRef<'a>),
    Repeated(RepeatedSequenceRef<'a>),
    Grouped(GroupedSequenceRef<'a>),
    MetaIdentifier(MetaIdentifierRef<'a>),
    Literal(LiteralRef<'a>),
    Empty,
}

impl<'a> IPrimary for PrimaryRef<'a> {
    type OptionalSequence = OptionalSequenceRef<'a>;
    type RepeatedSequence = RepeatedSequenceRef<'a>;
    type GroupedSequence = GroupedSequenceRef<'a>;
    type MetaIdentifier = MetaIdentifierRef<'a>;
    type Literal = LiteralRef<'a>;

    fn try_as_optional(&self) -> Option<&Self::OptionalSequence> {
        if let Self::Optional(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_repeated(&self) -> Option<&Self::RepeatedSequence> {
        if let Self::Repeated(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_grouped(&self) -> Option<&Self::GroupedSequence> {
        if let Self::Grouped(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_meta_identifier(&self) -> Option<&<PrimaryRef<'a> as IPrimary>::MetaIdentifier> {
        if let Self::MetaIdentifier(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }

    fn try_as_literal(&self) -> Option<&<PrimaryRef<'a> as IPrimary>::Literal> {
        if let Self::Literal(lit) = &self {
            Some(lit)
        } else {
            None
        }
    }

    fn kind(&self) -> PrimaryKind {
        match self {
            PrimaryRef::Optional(_) => PrimaryKind::OptionalSequence,
            PrimaryRef::Repeated(_) => PrimaryKind::RepeatedSequence,
            PrimaryRef::Grouped(_) => PrimaryKind::GroupedSequence,
            PrimaryRef::MetaIdentifier(_) => PrimaryKind::MetaId,
            PrimaryRef::Literal(_) => PrimaryKind::Literal,
            PrimaryRef::Empty => PrimaryKind::Empty,
        }
    }

    fn to_owned(self) -> Primary {
        match self {
            PrimaryRef::Optional(seq) => Primary::Optional(seq.to_owned()),
            PrimaryRef::Repeated(seq) => Primary::Repeated(seq.to_owned()),
            PrimaryRef::Grouped(seq) => Primary::Grouped(seq.to_owned()),
            PrimaryRef::MetaIdentifier(id) => {
                Primary::MetaIdentifier(IMetaIdentifier::to_owned(id))
            }
            PrimaryRef::Literal(lit) => Primary::Literal(ILiteral::to_owned(lit)),
            PrimaryRef::Empty => Primary::Empty,
        }
    }
}

pub enum PrimarySymbolIterator<'a> {
    Once(std::iter::Once<&'a Symbol>),
}

impl<'a> From<Once<&'a Symbol>> for PrimarySymbolIterator<'a> {
    fn from(value: Once<&'a Symbol>) -> Self {
        Self::Once(value)
    }
}

impl<'a> Iterator for PrimarySymbolIterator<'a> {
    type Item = &'a Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PrimarySymbolIterator::Once(iter) => iter.next(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primary {
    Optional(OptionalSequence),
    Repeated(RepeatedSequence),
    Grouped(GroupedSequence),
    MetaIdentifier(MetaIdentifier),
    Literal(Literal),
    Empty,
}

impl IntoTerm for Primary {
    type Term = Term;

    fn into_term(self) -> Self::Term {
        Term::from(self)
    }
}

impl From<MetaIdentifier> for Primary {
    fn from(value: MetaIdentifier) -> Self {
        Self::MetaIdentifier(value)
    }
}

impl<'a> IterSymbols<'a> for Primary {
    type Symbol = Symbol;
    type Iter = PrimarySymbolIterator<'a>;

    fn iter_symbols(&'a self) -> Self::Iter {
        match self {
            Primary::Optional(_) => todo!(),
            Primary::Repeated(_) => todo!(),
            Primary::Grouped(_) => todo!(),
            Primary::MetaIdentifier(meta) => {
                PrimarySymbolIterator::from(std::iter::once(meta.as_symbol()))
            }
            Primary::Literal(_) => todo!(),
            Primary::Empty => todo!(),
        }
    }
}

impl IPrimary for Primary {
    type OptionalSequence = OptionalSequence;
    type RepeatedSequence = RepeatedSequence;
    type GroupedSequence = GroupedSequence;
    type MetaIdentifier = MetaIdentifier;
    type Literal = Literal;

    fn try_as_optional(&self) -> Option<&Self::OptionalSequence> {
        if let Self::Optional(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_repeated(&self) -> Option<&Self::RepeatedSequence> {
        if let Self::Repeated(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_grouped(&self) -> Option<&Self::GroupedSequence> {
        if let Self::Grouped(seq) = &self {
            Some(seq)
        } else {
            None
        }
    }

    fn try_as_meta_identifier(&self) -> Option<&<Self as IPrimary>::MetaIdentifier> {
        if let Self::MetaIdentifier(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }

    fn try_as_literal(&self) -> Option<&<Self as IPrimary>::Literal> {
        if let Self::Literal(lit) = &self {
            Some(lit)
        } else {
            None
        }
    }

    fn kind(&self) -> PrimaryKind {
        match self {
            Self::Optional(_) => PrimaryKind::OptionalSequence,
            Self::Repeated(_) => PrimaryKind::RepeatedSequence,
            Self::Grouped(_) => PrimaryKind::GroupedSequence,
            Self::MetaIdentifier(_) => PrimaryKind::MetaId,
            Self::Literal(_) => PrimaryKind::Literal,
            Self::Empty => PrimaryKind::Empty,
        }
    }

    fn to_owned(self) -> Primary {
        self
    }
}

impl syn::parse::Parse for Primary {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if OptionalSequence::is_parsable(&input) {
            input.parse::<OptionalSequence>().map(Self::Optional)
        } else if RepeatedSequence::is_parsable(&input) {
            input.parse::<RepeatedSequence>().map(Self::Repeated)
        } else if GroupedSequence::is_parsable(&input) {
            input.parse::<GroupedSequence>().map(Self::Grouped)
        } else if Literal::is_parsable(&input) {
            input.parse::<Literal>().map(Self::Literal)
        } else if MetaIdentifier::is_parsable(&input) {
            input.parse::<MetaIdentifier>().map(Self::MetaIdentifier)
        } else {
            Ok(Self::Empty)
        }
    }
}
impl quote::ToTokens for Primary {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        tokens.extend(match &self {
            Primary::Optional(a) => quote! {::pb_ebnf::PrimaryRef::Optional(#a)},
            Primary::Repeated(a) => quote! {::pb_ebnf::PrimaryRef::Repeated(#a)},
            Primary::Grouped(a) => quote! {::pb_ebnf::PrimaryRef::Grouped(#a)},
            Primary::MetaIdentifier(a) => quote! {::pb_ebnf::PrimaryRef::MetaIdentifier(#a)},
            Primary::Literal(a) => quote! {::pb_ebnf::PrimaryRef::Literal(#a)},
            Primary::Empty => quote! {::pb_ebnf::PrimaryRef::Empty},
        })
    }
}
