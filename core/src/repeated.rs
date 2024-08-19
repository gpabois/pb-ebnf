use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsListRef, OwnedDefinitionsList},
    single_definition::OwnedSingleDefinition,
    SingleDefinitionRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> RepeatedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

impl<'a> Deref for RepeatedSequenceRef<'a> {
    type Target = [SingleDefinitionRef<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRepeatedSequence(OwnedDefinitionsList);

impl Deref for OwnedRepeatedSequence {
    type Target = [OwnedSingleDefinition];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl syn::parse::Parse for OwnedRepeatedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::braced;
        let content;
        braced!(content in input);
        let seq = content.parse::<OwnedDefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl quote::ToTokens for OwnedRepeatedSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::RepeatedSequenceRef::new(#a)})
    }
}
