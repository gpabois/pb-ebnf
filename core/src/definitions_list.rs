use std::ops::Deref;

use crate::{
    meta_identifier::MetaIdentifierRef,
    single_definition::{OwnedSingleDefinition, SingleDefinitionRef},
    BoxableSymbolIterator, SymbolIterable,
};

pub trait DefinitionsList: Deref<Target = [Self::SingleDefinition]> {
    type SingleDefinition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionsListRef<'a>(&'a [SingleDefinitionRef<'a>]);

impl<'a> DefinitionsListRef<'a> {
    pub const fn new(defs: &'a [SingleDefinitionRef<'a>]) -> Self {
        Self(defs)
    }

    pub fn transitive(&self) -> impl Iterator<Item = &MetaIdentifierRef<'a>> {
        self.iter().flat_map(SingleDefinitionRef::transitive)
    }
}

impl<'a> DefinitionsList for DefinitionsListRef<'a> {
    type SingleDefinition = SingleDefinitionRef<'a>;
}

impl<'a> SymbolIterable<'a> for &DefinitionsListRef<'a> {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|sd| sd.iter_symbols())
            .into_boxed_iterator()
    }
}

impl<'a> Deref for DefinitionsListRef<'a> {
    type Target = [SingleDefinitionRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedDefinitionsList(Vec<OwnedSingleDefinition>);

impl DefinitionsList for OwnedDefinitionsList {
    type SingleDefinition = OwnedSingleDefinition;
}

impl<'a> SymbolIterable<'a> for &'a OwnedDefinitionsList {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|sd| sd.iter_symbols())
            .into_boxed_iterator()
    }
}

impl Deref for OwnedDefinitionsList {
    type Target = [OwnedSingleDefinition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for OwnedDefinitionsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let mut list = vec![input.parse::<OwnedSingleDefinition>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<syn::Token![|]>().is_err() {
                break;
            }

            if let Ok(def) = fork.parse::<OwnedSingleDefinition>() {
                list.push(def);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl quote::ToTokens for OwnedDefinitionsList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let defs = crate::into_slice(self.0.iter());
        tokens.extend(quote::quote! {
            ::pb_ebnf::DefinitionsListRef::new(#defs)
        })
    }
}
