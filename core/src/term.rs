use std::ops::Deref;

use crate::{
    exception::{Exception, ExceptionRef, OwnedException},
    factor::{Factor, FactorRef, OwnedFactor},
    meta_identifier::MetaIdentifierRef,
};

pub trait Term: Deref<Target = Self::Factor> {
    type Factor: Factor;
    type Exception: Exception;

    fn get_factor(&self) -> &Self::Factor;
    fn get_exception(&self) -> Option<&Self::Exception>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermRef<'a> {
    pub factor: FactorRef<'a>,
    pub exception: Option<ExceptionRef<'a>>,
}

impl<'a> TermRef<'a> {
    pub const fn new(factor: FactorRef<'a>, exception: Option<ExceptionRef<'a>>) -> Self {
        Self { factor, exception }
    }

    pub fn try_as_single_symbol(&self) -> Option<&MetaIdentifierRef<'a>> {
        self.exception
            .is_none()
            .then(|| self.factor.try_as_single_symbol())
            .flatten()
    }
}

impl<'a> Deref for TermRef<'a> {
    type Target = FactorRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.factor
    }
}

impl<'a> Term for TermRef<'a> {
    type Factor = FactorRef<'a>;
    type Exception = ExceptionRef<'a>;

    fn get_factor(&self) -> &Self::Factor {
        &self.factor
    }

    fn get_exception(&self) -> Option<&Self::Exception> {
        self.exception.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedTerm {
    pub factor: OwnedFactor,
    pub exception: Option<OwnedException>,
}

impl Deref for OwnedTerm {
    type Target = OwnedFactor;

    fn deref(&self) -> &Self::Target {
        &self.factor
    }
}

impl Term for OwnedTerm {
    type Factor = OwnedFactor;
    type Exception = OwnedException;

    fn get_factor(&self) -> &Self::Factor {
        &self.factor
    }

    fn get_exception(&self) -> Option<&Self::Exception> {
        self.exception.as_ref()
    }
}

impl syn::parse::Parse for OwnedTerm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let factor = input.parse::<OwnedFactor>()?;
        let exception = if input.peek(syn::Token![-]) {
            input.parse::<OwnedException>().map(Some)?
        } else {
            None
        };

        Ok(Self { factor, exception })
    }
}
impl quote::ToTokens for OwnedTerm {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let factor = &self.factor;
        let exception = self
            .exception
            .as_ref()
            .map(|exception| quote::quote! {Some(#exception)})
            .unwrap_or(quote::quote! {None});

        tokens.extend(quote::quote! {::pb_ebnf::TermRef::new(#factor, #exception)});
    }
}
