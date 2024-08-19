use std::ops::Deref;

use crate::{
    meta_identifier::MetaIdentifierRef,
    primary::{OwnedPrimary, Primary, PrimaryRef},
};

pub trait Factor: Deref<Target = Self::Primary> {
    type Primary: Primary;

    fn get_repetition(&self) -> Option<u32>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FactorRef<'a> {
    pub primary: PrimaryRef<'a>,
    pub repetition: Option<u32>,
}

impl<'a> Deref for FactorRef<'a> {
    type Target = PrimaryRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.primary
    }
}

impl<'a> Factor for FactorRef<'a> {
    type Primary = PrimaryRef<'a>;

    fn get_repetition(&self) -> Option<u32> {
        self.repetition
    }
}

impl<'a> FactorRef<'a> {
    pub const fn new(primary: PrimaryRef<'a>, repetition: Option<u32>) -> Self {
        Self {
            primary,
            repetition,
        }
    }

    pub fn try_as_single_symbol(&self) -> Option<&MetaIdentifierRef<'a>> {
        self.repetition
            .is_none()
            .then(|| self.primary.try_as_symbol())
            .flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedFactor {
    pub primary: OwnedPrimary,
    pub repetition: Option<u32>,
}

impl Deref for OwnedFactor {
    type Target = OwnedPrimary;

    fn deref(&self) -> &Self::Target {
        &self.primary
    }
}

impl Factor for OwnedFactor {
    type Primary = OwnedPrimary;

    fn get_repetition(&self) -> Option<u32> {
        self.repetition
    }
}

impl syn::parse::Parse for OwnedFactor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        let repetition: Option<u32> = if input.peek(syn::LitInt) && input.peek2(Token![*]) {
            let rep = input.parse::<syn::LitInt>()?.base10_parse()?;
            input.parse::<Token![*]>()?;
            Some(rep)
        } else {
            None
        };

        input.parse::<OwnedPrimary>().map(move |primary| Self {
            repetition,
            primary,
        })
    }
}
impl quote::ToTokens for OwnedFactor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let repetition = self
            .repetition
            .map(|rep| quote! {Some(#rep)})
            .unwrap_or(quote! {None});
        let primary = &self.primary;

        tokens.extend(quote! {
            ::pb_ebnf::FactorRef::new(#primary, #repetition)
        })
    }
}
