use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsListRef, OwnedDefinitionsList},
    OwnedSingleDefinition, SingleDefinitionRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> GroupedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

impl<'a> Deref for GroupedSequenceRef<'a> {
    type Target = [SingleDefinitionRef<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedGroupedSequence(OwnedDefinitionsList);

impl Deref for OwnedGroupedSequence {
    type Target = [OwnedSingleDefinition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for OwnedGroupedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parenthesized;
        let content;
        parenthesized!(content in input);
        let seq = content.parse::<OwnedDefinitionsList>()?;
        Ok(Self(seq))
    }
}

impl quote::ToTokens for OwnedGroupedSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::GroupedSequenceRef::new(#a)})
    }
}
