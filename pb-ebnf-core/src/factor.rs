use crate::{
    primary::{Primary, PrimaryRef},
    symbol::SymbolRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FactorRef<'a> {
    pub primary: PrimaryRef<'a>,
    pub repetition: Option<u32>,
}
impl<'a> FactorRef<'a> {
    pub const fn new(primary: PrimaryRef<'a>, repetition: Option<u32>) -> Self {
        Self {
            primary,
            repetition,
        }
    }

    pub fn try_as_single_symbol(&self) -> Option<&SymbolRef<'a>> {
        self.repetition
            .is_none()
            .then(|| self.primary.try_as_symbol())
            .flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Factor {
    pub primary: Primary,
    pub repetition: Option<u32>,
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
