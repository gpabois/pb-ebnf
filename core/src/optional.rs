use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsListRef, OwnedDefinitionsList},
    single_definition::OwnedSingleDefinition,
    DefinitionsList, SingleDefinitionRef,
};

pub trait OptionalSequence:
    Deref<Target = [<Self::DefinitionsList as DefinitionsList>::SingleDefinition]>
{
    type DefinitionsList: DefinitionsList;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> OptionalSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

impl<'a> OptionalSequence for OptionalSequenceRef<'a> {
    type DefinitionsList = DefinitionsListRef<'a>;
}

impl<'a> Deref for OptionalSequenceRef<'a> {
    type Target = [SingleDefinitionRef<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedOptionalSequence(OwnedDefinitionsList);

impl Deref for OwnedOptionalSequence {
    type Target = [OwnedSingleDefinition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OptionalSequence for OwnedOptionalSequence {
    type DefinitionsList = OwnedDefinitionsList;
}

impl syn::parse::Parse for OwnedOptionalSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        pub use syn::bracketed;
        let content;
        bracketed!(content in input);
        let seq = content.parse::<OwnedDefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl quote::ToTokens for OwnedOptionalSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::OptionalSequenceRef::new(#a)})
    }
}
