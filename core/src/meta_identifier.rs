use pb_bnf::{
    prelude::*,
    symbol::{Symbol, SymbolFragment, SymbolRef},
};
use std::ops::Deref;

pub trait IMetaIdentifier: Deref<Target = str> {
    fn to_owned(self) -> MetaIdentifier;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaIdentifierRef<'a>(SymbolRef<'a>);

impl<'a> MetaIdentifierRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(SymbolRef::new(value))
    }
}

impl IMetaIdentifier for MetaIdentifierRef<'_> {
    fn to_owned(self) -> MetaIdentifier {
        MetaIdentifier(ISymbol::to_owned(&self.0))
    }
}

impl Deref for MetaIdentifierRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MetaIdentifier(Symbol);

impl MetaIdentifier {
    pub fn new<S: ToString>(id: S) -> Self {
        Self(Symbol::from(id.to_string()))
    }

    pub fn borrow(&self) -> MetaIdentifierRef<'_> {
        MetaIdentifierRef(ISymbol::borrow(&self.0))
    }

    pub fn as_symbol(&self) -> &Symbol {
        &self.0
    }

    pub fn into_symbol(self) -> Symbol {
        self.0
    }
}

impl IMetaIdentifier for MetaIdentifier {
    fn to_owned(self) -> MetaIdentifier {
        self
    }
}

impl Deref for MetaIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MetaIdentifier {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        SymbolFragment::is_parsable(input)
    }
}

impl syn::parse::Parse for MetaIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fragments = Vec::<SymbolFragment>::default();

        while SymbolFragment::is_parsable(&input) {
            fragments.push(input.parse()?);
        }

        let sym = Symbol::from(fragments.into_iter().fold(
            String::default(),
            |mut acc, fragment| {
                if acc.ends_with('-') {
                    acc.push_str(&fragment.into_string());
                } else {
                    acc.push(' ');
                    acc.push_str(&fragment.into_string());
                }

                acc
            },
        ));

        Ok(Self(sym))
    }
}
impl quote::ToTokens for MetaIdentifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let value = &self.0.to_string();
        tokens.extend(quote! {
            ::pb_ebnf::MetaIdentifierRef::new(#value)
        })
    }
}
