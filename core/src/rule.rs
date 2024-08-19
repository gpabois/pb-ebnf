use crate::{
    definitions_list::{DefinitionsListRef, OwnedDefinitionsList},
    meta_identifier::{MetaIdentifier, MetaIdentifierRef, OwnedMetaIdentifier},
    BoxableSymbolIterator, DefinitionsList, SymbolIterable,
};

/// A production rule.
pub trait Rule {
    type Lhs: MetaIdentifier;
    type Rhs: DefinitionsList;

    fn lhs(&self) -> &Self::Lhs;
    fn rhs(&self) -> &Self::Rhs;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleRef<'a> {
    pub lhs: MetaIdentifierRef<'a>,
    pub rhs: DefinitionsListRef<'a>,
}

impl<'a> Rule for RuleRef<'a> {
    type Lhs = MetaIdentifierRef<'a>;
    type Rhs = DefinitionsListRef<'a>;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

impl<'a> SymbolIterable<'a> for &RuleRef<'a> {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        std::iter::once(self.lhs.into())
            .chain(self.rhs.iter_symbols())
            .into_boxed_iterator()
    }
}

impl<'a> RuleRef<'a> {
    /// Creates a new rule.
    pub const fn new(lhs: MetaIdentifierRef<'a>, rhs: DefinitionsListRef<'a>) -> Self {
        Self { lhs, rhs }
    }

    /// Returns transitive symbols
    ///
    /// Transitive symbols are symbols which can directly produce another one.
    ///
    /// Example :
    /// <foo> = [<bar>] <acme>
    ///
    /// <acme> on its own produces <foo> as no other mandatory symbols are required.
    pub fn transitive(&self) -> impl Iterator<Item = &MetaIdentifierRef<'a>> {
        self.rhs.transitive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRule {
    pub lhs: OwnedMetaIdentifier,
    pub rhs: OwnedDefinitionsList,
}

impl<'a> SymbolIterable<'a> for &'a OwnedRule {
    fn iter_symbols(self) -> crate::SymbolIterator<'a> {
        std::iter::once((&self.lhs).into())
            .chain(self.rhs.iter_symbols())
            .into_boxed_iterator()
    }
}

impl Rule for OwnedRule {
    type Lhs = OwnedMetaIdentifier;
    type Rhs = OwnedDefinitionsList;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

impl syn::parse::Parse for OwnedRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = input.parse::<OwnedMetaIdentifier>()?;

        input
            .parse::<syn::Token![=]>()
            .map_err(|e| syn::Error::new(e.span(), "expecting a = after rule's lhs"))?;

        let rhs = input.parse::<OwnedDefinitionsList>()?;

        input
            .parse::<syn::Token![;]>()
            .map_err(|e| syn::Error::new(e.span(), "expecting a ;"))?;

        Ok(Self { lhs, rhs })
    }
}
impl quote::ToTokens for OwnedRule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let lhs = &self.lhs;
        let rhs = &self.rhs;

        tokens.extend(quote::quote! {
            ::pb_ebnf::RuleRef::new(#lhs, #rhs)
        })
    }
}
