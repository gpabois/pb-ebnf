use std::{marker::PhantomData, ops::Deref};

use crate::factor::{Factor, FactorRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionRef<'a>(FactorRef<'a>, PhantomData<&'a ()>);

impl<'a> ExceptionRef<'a> {
    pub const fn new(factor: FactorRef<'a>) -> Self {
        Self(factor, PhantomData)
    }
}

impl<'a> Deref for ExceptionRef<'a> {
    type Target = FactorRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exception(Factor);

impl syn::parse::Parse for Exception {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Factor>().map(Self)
    }
}

impl quote::ToTokens for Exception {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let factor = &self.0;
        tokens.extend(quote::quote! {
            ::pb_ebnf::ExceptionRef::new(#factor)
        })
    }
}
