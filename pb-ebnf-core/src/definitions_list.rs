use std::ops::Deref;

use crate::{
    single_definition::{SingleDefinition, SingleDefinitionRef},
    symbol::SymbolRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionsListRef<'a>(&'a [SingleDefinitionRef<'a>]);
impl<'a> DefinitionsListRef<'a> {
    pub const fn new(defs: &'a [SingleDefinitionRef<'a>]) -> Self {
        Self(defs)
    }

    pub fn transitive(&self) -> impl Iterator<Item = &SymbolRef<'a>> {
        self.iter().flat_map(SingleDefinitionRef::transitive)
    }
}
impl<'a> Deref for DefinitionsListRef<'a> {
    type Target = [SingleDefinitionRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionsList(Vec<SingleDefinition>);

impl Deref for DefinitionsList {
    type Target = [SingleDefinition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for DefinitionsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let mut list = vec![input.parse::<SingleDefinition>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<syn::Token![|]>().is_err() {
                break;
            }

            if let Ok(def) = fork.parse::<SingleDefinition>() {
                list.push(def);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl quote::ToTokens for DefinitionsList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let defs = crate::into_slice(self.0.iter());
        tokens.extend(quote::quote! {
            ::pb_ebnf::DefinitionsListRef::new(#defs)
        })
    }
}
