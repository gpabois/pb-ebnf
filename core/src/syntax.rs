use std::ops::Deref;

use itertools::Itertools;

use crate::{
    into_slice,
    rule::{OwnedRule, Rule, RuleRef},
    BoxableSymbolIterator, SymbolIterable, SymbolRef,
};

pub trait Syntax: std::ops::Deref<Target = [Self::Rule]> {
    type Rule: Rule;

    /// Iterate over rules based on its lhs symbol.
    fn iter_rules_by_symbol<S>(&self, symbol: S) -> impl Iterator<Item = &Self::Rule>
    where
        S: Deref<Target = str>,
    {
        self.iter()
            .filter(move |rule| rule.lhs().deref() == symbol.deref())
    }

    fn is_terminal<S>(&self, symbol: S) -> bool
    where
        S: Deref<Target = str>,
    {
        !self.is_non_terminal(symbol)
    }

    fn is_non_terminal<S>(&self, symbol: S) -> bool
    where
        S: Deref<Target = str>,
    {
        self.iter_rules_by_symbol(symbol).any(|_| true)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRef<'a>(&'a [RuleRef<'a>]);

impl<'a> Syntax for SyntaxRef<'a> {
    type Rule = RuleRef<'a>;
}

impl<'a> SyntaxRef<'a> {
    pub const fn new(rules: &'a [RuleRef<'a>]) -> Self {
        Self(rules)
    }

    pub fn iter_terminals(self) -> impl Iterator<Item = SymbolRef<'a>> {
        self.iter_symbols()
            .filter(move |sym| self.is_terminal(*sym))
            .into_boxed_iterator()
    }

    pub fn iter_non_terminals(self) -> impl Iterator<Item = SymbolRef<'a>> {
        self.iter_symbols()
            .filter(move |sym| self.is_non_terminal(*sym))
            .into_boxed_iterator()
    }
}

impl<'a> SymbolIterable<'a> for SyntaxRef<'a> {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|rule| rule.iter_symbols())
            .dedup()
            .into_boxed_iterator()
    }
}

impl<'a> Deref for SyntaxRef<'a> {
    type Target = [RuleRef<'a>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Default)]
pub struct OwnedSyntax(Vec<OwnedRule>);

impl FromIterator<OwnedRule> for OwnedSyntax {
    fn from_iter<T: IntoIterator<Item = OwnedRule>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Syntax for OwnedSyntax {
    type Rule = OwnedRule;
}

impl OwnedSyntax {
    pub fn iter_terminals(&self) -> impl Iterator<Item = SymbolRef<'_>> {
        self.iter_symbols().filter(|sym| self.is_terminal(*sym))
    }

    pub fn iter_non_terminals(&self) -> impl Iterator<Item = SymbolRef<'_>> {
        self.iter_symbols().filter(|sym| self.is_non_terminal(*sym))
    }
}

impl<'a> SymbolIterable<'a> for &'a OwnedSyntax {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        self.0
            .iter()
            .flat_map(|rule| rule.iter_symbols())
            .dedup()
            .into_boxed_iterator()
    }
}

impl Deref for OwnedSyntax {
    type Target = [OwnedRule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for OwnedSyntax {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ls = Vec::<OwnedRule>::default();

        while !input.is_empty() {
            ls.push(input.parse::<OwnedRule>()?);
        }

        Ok(Self(ls))
    }
}
impl quote::ToTokens for OwnedSyntax {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let rules = into_slice(self.0.iter());
        tokens.extend(quote::quote! {
            ::pb_ebnf::SyntaxRef::new(#rules)
        })
    }
}
