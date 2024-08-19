use std::ops::Deref;

use crate::{
    itertools::Single as _,
    meta_identifier::MetaIdentifierRef,
    term::{OwnedTerm, Term, TermRef},
    BoxableSymbolIterator, Factor, Primary, SymbolIterable,
};

pub trait SingleDefinition: Deref<Target = [Self::Term]> {
    type Term: Term;

    fn transitive(
        &self,
    ) -> Option<&<<<Self::Term as Term>::Factor as Factor>::Primary as Primary>::MetaIdentifier>
    {
        self.iter()
            .map(|term| term.try_as_meta_identifier())
            .filter(Option::is_some)
            .single()
            .flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleDefinitionRef<'a>(&'a [TermRef<'a>]);

impl<'a> SingleDefinitionRef<'a> {
    pub const fn new(terms: &'a [TermRef<'a>]) -> Self {
        Self(terms)
    }
}

impl<'a> SingleDefinitionRef<'a> {
    pub fn transitive(&self) -> Option<&MetaIdentifierRef<'a>> {
        self.iter()
            .map(TermRef::try_as_single_symbol)
            .filter(Option::is_some)
            .single()
            .flatten()
    }
}

impl<'a> SymbolIterable<'a> for SingleDefinitionRef<'a> {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|term| term.iter_symbols())
            .into_boxed_iterator()
    }
}

impl<'a> SymbolIterable<'a> for &SingleDefinitionRef<'a> {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|term| term.iter_symbols())
            .into_boxed_iterator()
    }
}

impl<'a> SingleDefinition for SingleDefinitionRef<'a> {
    type Term = TermRef<'a>;
}

impl<'a> Deref for SingleDefinitionRef<'a> {
    type Target = [TermRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedSingleDefinition(Vec<OwnedTerm>);

impl<'a> SymbolIterable<'a> for &'a OwnedSingleDefinition {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|term| term.iter_symbols())
            .into_boxed_iterator()
    }
}

impl SingleDefinition for OwnedSingleDefinition {
    type Term = OwnedTerm;
}

impl Deref for OwnedSingleDefinition {
    type Target = [OwnedTerm];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl syn::parse::Parse for OwnedSingleDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative as _;
        use syn::Token;

        let mut list = vec![input.parse::<OwnedTerm>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<Token![,]>().is_err() {
                break;
            }

            if let Ok(term) = fork.parse::<OwnedTerm>() {
                list.push(term);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl quote::ToTokens for OwnedSingleDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let terms = crate::into_slice(self.0.iter());
        tokens.extend(quote! {
            ::pb_ebnf::SingleDefinitionRef::new(#terms)
        })
    }
}
