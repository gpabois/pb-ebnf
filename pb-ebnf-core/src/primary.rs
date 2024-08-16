use crate::{
    grouped::{GroupedSequence, GroupedSequenceRef},
    literal::{Literal, LiteralRef},
    optional::{OptionalSequence, OptionalSequenceRef},
    repeated::{RepeatedSequence, RepeatedSequenceRef},
    symbol::{Symbol, SymbolRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryRef<'a> {
    Optional(OptionalSequenceRef<'a>),
    Repeated(RepeatedSequenceRef<'a>),
    Grouped(GroupedSequenceRef<'a>),
    Symbol(SymbolRef<'a>),
    Literal(LiteralRef<'a>),
    Empty,
}

impl<'a> PrimaryRef<'a> {
    #[inline]
    pub fn try_as_symbol(&self) -> Option<&SymbolRef<'a>> {
        if let Self::Symbol(sym) = &self {
            Some(sym)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primary {
    Optional(OptionalSequence),
    Repeated(RepeatedSequence),
    Grouped(GroupedSequence),
    Symbol(Symbol),
    Literal(Literal),
    Empty,
}

impl syn::parse::Parse for Primary {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::token::{Brace, Bracket, Paren};

        // [
        if input.peek(Bracket) {
            input.parse::<OptionalSequence>().map(Self::Optional)
        } else if input.peek(Brace) {
            input.parse::<RepeatedSequence>().map(Self::Repeated)
        } else if input.peek(Paren) {
            input.parse::<GroupedSequence>().map(Self::Grouped)
        } else if input.peek(syn::LitStr) {
            input.parse::<Literal>().map(Self::Literal)
        } else if Symbol::is_beginning_of_symbol(&input) {
            input.parse::<Symbol>().map(Self::Symbol)
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
            Primary::Symbol(a) => quote! {::pb_ebnf::PrimaryRef::Symbol(#a)},
            Primary::Literal(a) => quote! {::pb_ebnf::PrimaryRef::Literal(#a)},
            Primary::Empty => quote! {::pb_ebnf::PrimaryRef::Empty},
        })
    }
}
