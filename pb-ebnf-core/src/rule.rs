use crate::{
    definitions_list::{DefinitionsList, DefinitionsListRef},
    symbol::{Symbol, SymbolRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleRef<'a> {
    pub lhs: SymbolRef<'a>,
    pub rhs: DefinitionsListRef<'a>,
}

impl<'a> RuleRef<'a> {
    /// Creates a new rule.
    pub const fn new(lhs: SymbolRef<'a>, rhs: DefinitionsListRef<'a>) -> Self {
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
    pub fn transitive(&self) -> impl Iterator<Item = &SymbolRef<'a>> {
        self.rhs.transitive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub lhs: Symbol,
    pub rhs: DefinitionsList,
}

impl syn::parse::Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = input.parse::<Symbol>()?;

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
