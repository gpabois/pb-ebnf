use std::{marker::PhantomData, ops::Deref};

use crate::factor::{FactorRef, OwnedFactor};

pub trait Exception: Deref<Target = Self::Factor> {
    type Factor;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionRef<'a>(FactorRef<'a>, PhantomData<&'a ()>);

impl<'a> ExceptionRef<'a> {
    pub const fn new(factor: FactorRef<'a>) -> Self {
        Self(factor, PhantomData)
    }
}

impl<'a> Exception for ExceptionRef<'a> {
    type Factor = FactorRef<'a>;
}

impl<'a> Deref for ExceptionRef<'a> {
    type Target = FactorRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedException(OwnedFactor);

impl Exception for OwnedException {
    type Factor = OwnedFactor;
}

impl Deref for OwnedException {
    type Target = OwnedFactor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for OwnedException {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<OwnedFactor>().map(Self)
    }
}

impl quote::ToTokens for OwnedException {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let factor = &self.0;
        tokens.extend(quote::quote! {
            ::pb_ebnf::ExceptionRef::new(#factor)
        })
    }
}
