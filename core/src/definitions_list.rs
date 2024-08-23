use std::ops::{Deref, DerefMut};

use pb_bnf::symbol::Symbol;

use crate::{
    prelude::*,
    single_definition::{SingleDefinition, SingleDefinitionRef},
};

pub trait IDefinitionsList: AsRef<[Self::SingleDefinition]> + Clone {
    type SingleDefinition: ISingleDefinition;

    fn to_owned(self) -> DefinitionsList;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionsListRef<'a>(&'a [SingleDefinitionRef<'a>]);

impl<'a> DefinitionsListRef<'a> {
    pub const fn new(defs: &'a [SingleDefinitionRef<'a>]) -> Self {
        Self(defs)
    }
}

impl<'a> IDefinitionsList for DefinitionsListRef<'a> {
    type SingleDefinition = SingleDefinitionRef<'a>;

    fn to_owned(self) -> DefinitionsList {
        DefinitionsList(
            self.0
                .iter()
                .copied()
                .map(ISingleDefinition::to_owned)
                .collect(),
        )
    }
}

impl<'a> AsRef<[SingleDefinitionRef<'a>]> for DefinitionsListRef<'a> {
    fn as_ref(&self) -> &[SingleDefinitionRef<'a>] {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionsList(Vec<SingleDefinition>);

impl<'a> IterSymbols<'a> for DefinitionsList {
    type Symbol = Symbol;
    type Iter = Box<dyn Iterator<Item = &'a Symbol> + 'a>;

    fn iter_symbols(&'a self) -> Self::Iter {
        let iter = self
            .iter()
            .flat_map(|def| def.iter().flat_map(|term| term.iter_symbols()));

        Box::new(iter)
    }
}

impl IntoIterator for DefinitionsList {
    type Item = SingleDefinition;

    type IntoIter = <Vec<SingleDefinition> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<SingleDefinition> for DefinitionsList {
    fn from_iter<T: IntoIterator<Item = SingleDefinition>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Deref for DefinitionsList {
    type Target = Vec<SingleDefinition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefinitionsList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[SingleDefinition]> for DefinitionsList {
    fn as_ref(&self) -> &[SingleDefinition] {
        self.0.as_slice()
    }
}

impl IDefinitionsList for DefinitionsList {
    type SingleDefinition = SingleDefinition;

    fn to_owned(self) -> DefinitionsList {
        self
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
