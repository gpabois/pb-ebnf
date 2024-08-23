use std::ops::{Deref, DerefMut};

use crate::definitions_list::{DefinitionsList, DefinitionsListRef, IDefinitionsList};

pub trait IGroupedSequence: AsRef<Self::DefinitionsList> {
    type DefinitionsList: IDefinitionsList;

    fn to_owned(self) -> GroupedSequence;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedSequenceRef<'a>(DefinitionsListRef<'a>);

impl<'a> IGroupedSequence for GroupedSequenceRef<'a> {
    type DefinitionsList = DefinitionsListRef<'a>;

    fn to_owned(self) -> GroupedSequence {
        GroupedSequence(self.0.to_owned())
    }
}

impl<'a> GroupedSequenceRef<'a> {
    pub const fn new(defs: DefinitionsListRef<'a>) -> Self {
        Self(defs)
    }
}

impl<'a> AsRef<DefinitionsListRef<'a>> for GroupedSequenceRef<'a> {
    fn as_ref(&self) -> &DefinitionsListRef<'a> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedSequence(DefinitionsList);

impl GroupedSequence {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        input.peek(syn::token::Paren)
    }
    pub fn into_definitions_list(self) -> DefinitionsList {
        self.0
    }
}

impl AsRef<DefinitionsList> for GroupedSequence {
    fn as_ref(&self) -> &DefinitionsList {
        &self.0
    }
}

impl IGroupedSequence for GroupedSequence {
    type DefinitionsList = DefinitionsList;

    fn to_owned(self) -> GroupedSequence {
        self
    }
}

impl Deref for GroupedSequence {
    type Target = DefinitionsList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GroupedSequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
