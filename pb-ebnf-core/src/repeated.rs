use std::ops::Deref;

use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef},
    single_definition::SingleDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> RepeatedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatedSequence(DefinitionsList);

impl Deref for RepeatedSequence {
    type Target = [SingleDefinition];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
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
