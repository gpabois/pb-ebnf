use crate::definitions_list::{DefinitionsList, DefinitionsListRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedSequenceRef<'a>(DefinitionsListRef<'a>);
impl<'a> GroupedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedSequence(DefinitionsList);

impl syn::parse::Parse for GroupedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parenthesized;
        let content;
        parenthesized!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl quote::ToTokens for GroupedSequence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::GroupedSequenceRef::new(#a)})
    }
}
