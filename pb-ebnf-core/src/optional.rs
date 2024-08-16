use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef},
    single_definition::SingleDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> OptionalSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalSequence(DefinitionsList);

impl Deref for OptionalSequence {
    type Target = [SingleDefinition];

    fn deref(&self) -> &Self::Target {
        &self.0
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
