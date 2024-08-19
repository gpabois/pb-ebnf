use crate::{
    grouped::{GroupedSequenceRef, OwnedGroupedSequence},
    literal::{LiteralRef, OwnedLiteral},
    meta_identifier::{MetaIdentifierRef, OwnedMetaIdentifier},
    optional::{OptionalSequenceRef, OwnedOptionalSequence},
    repeated::{OwnedRepeatedSequence, RepeatedSequenceRef},
    BoxableSymbolIterator, SymbolIterable, SymbolIterator, SymbolRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryKind {
    OptionalSequence,
    RepeatedSequence,
    GroupedSequence,
    MetaId,
    Literal,
    Empty,
}

pub trait Primary {
    type OptionalSequence;
    type RepeatedSequence;
    type GroupedSequence;
    type MetaIdentifier;
    type Literal;

    fn try_as_optional(&self) -> Option<&Self::OptionalSequence>;
    fn try_as_repeated(&self) -> Option<&Self::RepeatedSequence>;
    fn try_as_grouped(&self) -> Option<&Self::GroupedSequence>;
    fn try_as_meta_identifier(&self) -> Option<&Self::MetaIdentifier>;
    fn try_as_literal(&self) -> Option<&Self::Literal>;

    fn kind(&self) -> PrimaryKind;
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

impl<'a> Primary for PrimaryRef<'a> {
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

    fn try_as_meta_identifier(&self) -> Option<&<PrimaryRef<'a> as Primary>::MetaIdentifier> {
        if let Self::MetaIdentifier(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }

    fn try_as_literal(&self) -> Option<&<PrimaryRef<'a> as Primary>::Literal> {
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
}

impl<'a> SymbolIterable<'a> for &'a PrimaryRef<'a> {
    fn iter_symbols(self) -> SymbolIterator<'a> {
        match self {
            PrimaryRef::Optional(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            PrimaryRef::Repeated(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            PrimaryRef::Grouped(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            PrimaryRef::MetaIdentifier(id) => std::iter::once(SymbolRef::from(*id)).into(),
            PrimaryRef::Literal(lit) => std::iter::once(SymbolRef::from(*lit)).into(),
            PrimaryRef::Empty => std::iter::empty().into(),
        }
    }
}

impl<'a> PrimaryRef<'a> {
    #[inline]
    pub fn try_as_symbol(&self) -> Option<&MetaIdentifierRef<'a>> {
        if let Self::MetaIdentifier(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnedPrimary {
    Optional(OwnedOptionalSequence),
    Repeated(OwnedRepeatedSequence),
    Grouped(OwnedGroupedSequence),
    MetaIdentifier(OwnedMetaIdentifier),
    Literal(OwnedLiteral),
    Empty,
}

impl Primary for OwnedPrimary {
    type OptionalSequence = OwnedOptionalSequence;
    type RepeatedSequence = OwnedRepeatedSequence;
    type GroupedSequence = OwnedGroupedSequence;
    type MetaIdentifier = OwnedMetaIdentifier;
    type Literal = OwnedLiteral;

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

    fn try_as_meta_identifier(&self) -> Option<&<Self as Primary>::MetaIdentifier> {
        if let Self::MetaIdentifier(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }

    fn try_as_literal(&self) -> Option<&<Self as Primary>::Literal> {
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
}

impl<'a> SymbolIterable<'a> for &'a OwnedPrimary {
    fn iter_symbols(self) -> SymbolIterator<'a> {
        match self {
            OwnedPrimary::Optional(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            OwnedPrimary::Repeated(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            OwnedPrimary::Grouped(seq) => seq
                .iter()
                .flat_map(|sd| sd.iter_symbols())
                .into_boxed_iterator(),
            OwnedPrimary::MetaIdentifier(id) => std::iter::once(SymbolRef::from(id)).into(),
            OwnedPrimary::Literal(lit) => std::iter::once(SymbolRef::from(lit)).into(),
            OwnedPrimary::Empty => std::iter::empty().into(),
        }
    }
}

impl syn::parse::Parse for OwnedPrimary {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::token::{Brace, Bracket, Paren};

        // [
        if input.peek(Bracket) {
            input.parse::<OwnedOptionalSequence>().map(Self::Optional)
        } else if input.peek(Brace) {
            input.parse::<OwnedRepeatedSequence>().map(Self::Repeated)
        } else if input.peek(Paren) {
            input.parse::<OwnedGroupedSequence>().map(Self::Grouped)
        } else if input.peek(syn::LitStr) || input.peek(syn::LitChar) {
            input.parse::<OwnedLiteral>().map(Self::Literal)
        } else if OwnedMetaIdentifier::is_beginning_of_symbol(&input) {
            input
                .parse::<OwnedMetaIdentifier>()
                .map(Self::MetaIdentifier)
        } else {
            Ok(Self::Empty)
        }
    }
}
impl quote::ToTokens for OwnedPrimary {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        tokens.extend(match &self {
            OwnedPrimary::Optional(a) => quote! {::pb_ebnf::PrimaryRef::Optional(#a)},
            OwnedPrimary::Repeated(a) => quote! {::pb_ebnf::PrimaryRef::Repeated(#a)},
            OwnedPrimary::Grouped(a) => quote! {::pb_ebnf::PrimaryRef::Grouped(#a)},
            OwnedPrimary::MetaIdentifier(a) => quote! {::pb_ebnf::PrimaryRef::MetaIdentifier(#a)},
            OwnedPrimary::Literal(a) => quote! {::pb_ebnf::PrimaryRef::Literal(#a)},
            OwnedPrimary::Empty => quote! {::pb_ebnf::PrimaryRef::Empty},
        })
    }
}
