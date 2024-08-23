use std::ops::{Deref, DerefMut};

use crate::{
    prelude::*,
    primary::Primary,
    term::{Term, TermRef},
};

pub trait ISingleDefinition: AsRef<[Self::Term]> + Clone {
    type Term: ITerm;

    fn to_owned(self) -> SingleDefinition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleDefinitionRef<'a>(&'a [TermRef<'a>]);

impl<'a> SingleDefinitionRef<'a> {
    pub const fn new(terms: &'a [TermRef<'a>]) -> Self {
        Self(terms)
    }
}

impl<'a> ISingleDefinition for SingleDefinitionRef<'a> {
    type Term = TermRef<'a>;

    fn to_owned(self) -> SingleDefinition {
        SingleDefinition(self.0.iter().copied().map(ITerm::to_owned).collect())
    }
}

impl<'a> AsRef<[TermRef<'a>]> for SingleDefinitionRef<'a> {
    fn as_ref(&self) -> &[TermRef<'a>] {
        self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SingleDefinition(Vec<Term>);

impl SingleDefinition {
    pub fn push_front(&mut self, term: Term) {
        self.0.insert(0, term)
    }

    pub fn empty() -> Self {
        Self(vec![Term::from(Primary::Empty)])
    }
}

impl ISingleDefinition for SingleDefinition {
    type Term = Term;

    fn to_owned(self) -> SingleDefinition {
        self
    }
}

impl AsRef<[Term]> for SingleDefinition {
    fn as_ref(&self) -> &[Term] {
        self.0.as_slice()
    }
}

impl FromIterator<Term> for SingleDefinition {
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for SingleDefinition {
    type Item = Term;
    type IntoIter = <Vec<Term> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for SingleDefinition {
    type Target = Vec<Term>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SingleDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
