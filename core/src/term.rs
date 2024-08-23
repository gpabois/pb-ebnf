use std::ops::Deref;

use crate::{
    exception::{Exception, ExceptionRef, IException},
    factor::{Factor, FactorRef, IFactor},
    primary::Primary,
    IntoPrimary,
};

pub trait IntoTerm {
    type Term: ITerm;

    fn into_term(self) -> Self::Term;
}

pub trait ITerm: Deref<Target = Self::Factor> + Clone {
    type Factor: IFactor;
    type Exception: IException;

    fn get_factor(&self) -> &Self::Factor;
    fn get_exception(&self) -> Option<&Self::Exception>;

    fn to_owned(self) -> Term;
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
}

impl<'a> Deref for TermRef<'a> {
    type Target = FactorRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.factor
    }
}

impl<'a> ITerm for TermRef<'a> {
    type Factor = FactorRef<'a>;
    type Exception = ExceptionRef<'a>;

    fn get_factor(&self) -> &Self::Factor {
        &self.factor
    }

    fn get_exception(&self) -> Option<&Self::Exception> {
        self.exception.as_ref()
    }

    fn to_owned(self) -> Term {
        Term {
            factor: self.factor.to_owned(),
            exception: self.exception.map(IException::to_owned),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub factor: Factor,
    pub exception: Option<Exception>,
}

impl IntoPrimary for Term {
    type Primary = <Factor as IntoPrimary>::Primary;

    fn into_primary(self) -> Self::Primary {
        self.factor.into_primary()
    }
}

impl From<Primary> for Term {
    fn from(value: Primary) -> Self {
        Self::from(Factor::from(value))
    }
}

impl From<Factor> for Term {
    fn from(value: Factor) -> Self {
        Self {
            factor: value,
            exception: None,
        }
    }
}

impl Deref for Term {
    type Target = Factor;

    fn deref(&self) -> &Self::Target {
        &self.factor
    }
}

impl ITerm for Term {
    type Factor = Factor;
    type Exception = Exception;

    fn get_factor(&self) -> &Self::Factor {
        &self.factor
    }

    fn get_exception(&self) -> Option<&Self::Exception> {
        self.exception.as_ref()
    }

    fn to_owned(self) -> Term {
        self
    }
}

impl syn::parse::Parse for Term {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let factor = input.parse::<Factor>()?;
        let exception = if input.peek(syn::Token![-]) {
            input.parse::<Exception>().map(Some)?
        } else {
            None
        };

        Ok(Self { factor, exception })
    }
}
impl quote::ToTokens for Term {
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
