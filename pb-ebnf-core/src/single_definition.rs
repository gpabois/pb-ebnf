use std::ops::Deref;

use crate::{
    itertools::Single as _,
    symbol::SymbolRef,
    term::{Term, TermRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleDefinitionRef<'a>(&'a [TermRef<'a>]);
impl<'a> SingleDefinitionRef<'a> {
    pub const fn new(terms: &'a [TermRef<'a>]) -> Self {
        Self(terms)
    }
}
impl<'a> SingleDefinitionRef<'a> {
    pub fn transitive(&self) -> Option<&SymbolRef<'a>> {
        self.iter()
            .map(TermRef::try_as_single_symbol)
            .filter(Option::is_some)
            .single()
            .flatten()
    }
}
impl<'a> Deref for SingleDefinitionRef<'a> {
    type Target = [TermRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleDefinition(Vec<Term>);

impl syn::parse::Parse for SingleDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative as _;
        use syn::Token;

        let mut list = vec![input.parse::<Term>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<Token![,]>().is_err() {
                break;
            }

            if let Ok(term) = fork.parse::<Term>() {
                list.push(term);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl quote::ToTokens for SingleDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let terms = crate::into_slice(self.0.iter());
        tokens.extend(quote! {
            ::pb_ebnf::SingleDefinitionRef::new(#terms)
        })
    }
}
