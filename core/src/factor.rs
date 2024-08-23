use crate::{
    primary::{IPrimary, Primary, PrimaryRef},
    AsPrimaryRef, IntoPrimary,
};
use std::ops::Deref;

pub trait IFactor: Deref<Target = Self::Primary> {
    type Primary: IPrimary;

    fn get_repetition(&self) -> Option<u32>;
    fn to_owned(self) -> Factor;
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

impl<'a> AsRef<PrimaryRef<'a>> for FactorRef<'a> {
    fn as_ref(&self) -> &PrimaryRef<'a> {
        &self.primary
    }
}

impl<'a> IFactor for FactorRef<'a> {
    type Primary = PrimaryRef<'a>;

    fn get_repetition(&self) -> Option<u32> {
        self.repetition
    }

    fn to_owned(self) -> Factor {
        Factor {
            primary: self.primary.to_owned(),
            repetition: self.repetition,
        }
    }
}

impl<'a> FactorRef<'a> {
    pub const fn new(primary: PrimaryRef<'a>, repetition: Option<u32>) -> Self {
        Self {
            primary,
            repetition,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Factor {
    pub primary: Primary,
    pub repetition: Option<u32>,
}

impl AsPrimaryRef for Factor {
    type Primary = Primary;

    fn as_primary_ref(&self) -> &Self::Primary {
        &self.primary
    }
}

impl IntoPrimary for Factor {
    type Primary = Primary;

    fn into_primary(self) -> Self::Primary {
        self.primary
    }
}

impl From<Primary> for Factor {
    fn from(value: Primary) -> Self {
        Self {
            primary: value,
            repetition: None,
        }
    }
}

impl Deref for Factor {
    type Target = Primary;

    fn deref(&self) -> &Self::Target {
        &self.primary
    }
}

impl AsRef<Primary> for Factor {
    fn as_ref(&self) -> &Primary {
        &self.primary
    }
}

impl IFactor for Factor {
    type Primary = Primary;

    fn get_repetition(&self) -> Option<u32> {
        self.repetition
    }

    fn to_owned(self) -> Factor {
        self
    }
}

impl syn::parse::Parse for Factor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        let repetition: Option<u32> = if input.peek(syn::LitInt) && input.peek2(Token![*]) {
            let rep = input.parse::<syn::LitInt>()?.base10_parse()?;
            input.parse::<Token![*]>()?;
            Some(rep)
        } else {
            None
        };

        input.parse::<Primary>().map(move |primary| Self {
            repetition,
            primary,
        })
    }
}
impl quote::ToTokens for Factor {
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
