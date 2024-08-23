use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef},
    prelude::*,
};

pub trait IOptionalSequence: Deref<Target = Self::DefinitionsList> {
    type DefinitionsList: IDefinitionsList;

    fn to_owned(self) -> OptionalSequence;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> OptionalSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

impl<'a> Deref for OptionalSequenceRef<'a> {
    type Target = DefinitionsListRef<'a>;

    fn deref(&self) -> &DefinitionsListRef<'a> {
        &self.0
    }
}

impl<'a> IOptionalSequence for OptionalSequenceRef<'a> {
    type DefinitionsList = DefinitionsListRef<'a>;

    fn to_owned(self) -> OptionalSequence {
        OptionalSequence(self.0.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalSequence(DefinitionsList);

impl OptionalSequence {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        input.peek(syn::token::Bracket)
    }

    pub fn into_definitions_list(self) -> DefinitionsList {
        self.0
    }
}

impl Deref for OptionalSequence {
    type Target = DefinitionsList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IOptionalSequence for OptionalSequence {
    type DefinitionsList = DefinitionsList;

    fn to_owned(self) -> OptionalSequence {
        self
    }
}

impl syn::parse::Parse for OptionalSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        pub use syn::bracketed;
        let content;
        bracketed!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl quote::ToTokens for OptionalSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::OptionalSequenceRef::new(#a)})
    }
}
