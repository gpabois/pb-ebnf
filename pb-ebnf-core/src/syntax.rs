use std::ops::Deref;

use crate::{
    into_slice,
    rule::{Rule, RuleRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRef<'a>(&'a [RuleRef<'a>]);

impl<'a> SyntaxRef<'a> {
    pub const fn new(rules: &'a [RuleRef<'a>]) -> Self {
        Self(rules)
    }
}

impl<'a> Deref for SyntaxRef<'a> {
    type Target = [RuleRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Default)]
pub struct Syntax(Vec<Rule>);

impl FromIterator<Rule> for Syntax {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Deref for Syntax {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for Syntax {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ls = Vec::<Rule>::default();

        while !input.is_empty() {
            ls.push(input.parse::<Rule>()?);
        }

        Ok(Self(ls))
    }
}
impl quote::ToTokens for Syntax {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let rules = into_slice(self.0.iter());
        tokens.extend(quote::quote! {
            ::pb_ebnf::SyntaxRef::new(#rules)
        })
    }
}
