use std::ops::{Deref, DerefMut};

use crate::{
    into_slice,
    rule::{IRule, Rule, RuleRef},
};

pub trait ISyntax: AsRef<[Self::Rule]> {
    type Rule: IRule;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRef<'a>(&'a [RuleRef<'a>]);

impl<'a> ISyntax for SyntaxRef<'a> {
    type Rule = RuleRef<'a>;
}

impl<'a> SyntaxRef<'a> {
    pub const fn new(rules: &'a [RuleRef<'a>]) -> Self {
        Self(rules)
    }
}

impl<'a> AsRef<[RuleRef<'a>]> for SyntaxRef<'a> {
    fn as_ref(&self) -> &[RuleRef<'a>] {
        self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Syntax(Vec<Rule>);

impl IntoIterator for Syntax {
    type Item = Rule;

    type IntoIter = <Vec<Rule> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl FromIterator<Rule> for Syntax {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl ISyntax for Syntax {
    type Rule = Rule;
}

impl Syntax {
    pub fn push(&mut self, rule: Rule) {
        self.0.push(rule)
    }
}

impl Deref for Syntax {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Syntax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[Rule]> for Syntax {
    fn as_ref(&self) -> &[Rule] {
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
