use std::ops::Deref;

use pb_bnf::{
    prelude::*,
    symbol::{Symbol, SymbolRef},
};

pub trait ILiteral: Deref<Target = str> {
    fn to_owned(self) -> Literal;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralRef<'a>(SymbolRef<'a>);

impl<'a> LiteralRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(SymbolRef::new(value))
    }
}

impl<'a> ILiteral for LiteralRef<'a> {
    fn to_owned(self) -> Literal {
        Literal(ISymbol::to_owned(&self.0))
    }
}

impl Deref for LiteralRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal(Symbol);

impl Literal {
    pub fn new<S: ToString>(value: S) -> Self {
        Self(Symbol::from(value.to_string()))
    }

    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        use syn::{LitChar, LitStr};
        input.peek(LitChar) || input.peek(LitStr)
    }

    pub fn borrow(&self) -> LiteralRef<'_> {
        LiteralRef(ISymbol::borrow(&self.0))
    }

    pub fn into_symbol(self) -> Symbol {
        self.0
    }
}

impl ILiteral for Literal {
    fn to_owned(self) -> Literal {
        self
    }
}

impl Deref for Literal {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for Literal {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitChar) {
            input
                .parse::<syn::LitChar>()
                .map(|s| Self::new(s.value().to_string()))
        } else {
            input.parse::<syn::LitStr>().map(|s| Self::new(s.value()))
        }
    }
}

impl quote::ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let value = &self.0.to_string();
        tokens.extend(quote! {
            ::pb_ebnf::LiteralRef::new(#value)
        })
    }
}
