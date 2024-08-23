use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef},
    meta_identifier::{MetaIdentifier, MetaIdentifierRef},
    prelude::*,
};

/// A production rule.
pub trait IRule: Clone {
    type Lhs: IMetaIdentifier;
    type Rhs: IDefinitionsList;

    fn lhs(&self) -> &Self::Lhs;
    fn rhs(&self) -> &Self::Rhs;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleRef<'a> {
    pub lhs: MetaIdentifierRef<'a>,
    pub rhs: DefinitionsListRef<'a>,
}

impl<'a> IRule for RuleRef<'a> {
    type Lhs = MetaIdentifierRef<'a>;
    type Rhs = DefinitionsListRef<'a>;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

impl<'a> RuleRef<'a> {
    /// Creates a new rule.
    pub const fn new(lhs: MetaIdentifierRef<'a>, rhs: DefinitionsListRef<'a>) -> Self {
        Self { lhs, rhs }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub lhs: MetaIdentifier,
    pub rhs: DefinitionsList,
}

impl Rule {
    pub fn new(lhs: MetaIdentifier, rhs: DefinitionsList) -> Self {
        Self { lhs, rhs }
    }
}

impl IRule for Rule {
    type Lhs = MetaIdentifier;
    type Rhs = DefinitionsList;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

impl syn::parse::Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = input.parse::<MetaIdentifier>()?;

        input
            .parse::<syn::Token![=]>()
            .map_err(|e| syn::Error::new(e.span(), "expecting a = after rule's lhs"))?;

        let rhs = input.parse::<DefinitionsList>()?;

        input
            .parse::<syn::Token![;]>()
            .map_err(|e| syn::Error::new(e.span(), "expecting a ;"))?;

        Ok(Self { lhs, rhs })
    }
}
impl quote::ToTokens for Rule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let lhs = &self.lhs;
        let rhs = &self.rhs;

        tokens.extend(quote::quote! {
            ::pb_ebnf::RuleRef::new(#lhs, #rhs)
        })
    }
}
