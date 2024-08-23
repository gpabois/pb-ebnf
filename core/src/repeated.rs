use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef, IDefinitionsList},
    single_definition::SingleDefinition,
};

pub trait IRepeatedSequence {
    fn to_owned(self) -> RepeatedSequence;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> IRepeatedSequence for RepeatedSequenceRef<'a> {
    fn to_owned(self) -> RepeatedSequence {
        RepeatedSequence(self.0.to_owned())
    }
}

impl<'a> RepeatedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }

    pub fn to_owned(self) -> RepeatedSequence {
        RepeatedSequence(self.0.to_owned())
    }
}

impl<'a> AsRef<DefinitionsListRef<'a>> for RepeatedSequenceRef<'a> {
    fn as_ref(&self) -> &DefinitionsListRef<'a> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatedSequence(DefinitionsList);

impl RepeatedSequence {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        input.peek(syn::token::Brace)
    }

    pub fn into_definitions_list(self) -> DefinitionsList {
        self.0
    }
}

impl Deref for RepeatedSequence {
    type Target = [SingleDefinition];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl IRepeatedSequence for RepeatedSequence {
    fn to_owned(self) -> RepeatedSequence {
        self
    }
}

impl syn::parse::Parse for RepeatedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::braced;
        let content;
        braced!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl quote::ToTokens for RepeatedSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::RepeatedSequenceRef::new(#a)})
    }
}
