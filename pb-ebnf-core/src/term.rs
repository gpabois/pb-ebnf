use crate::{
    exception::{Exception, ExceptionRef},
    factor::{Factor, FactorRef},
    symbol::SymbolRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermRef<'a> {
    pub factor: FactorRef<'a>,
    pub exception: Option<ExceptionRef<'a>>,
}

impl<'a> TermRef<'a> {
    pub const fn new(factor: FactorRef<'a>, exception: Option<ExceptionRef<'a>>) -> Self {
        Self { factor, exception }
    }

    pub fn try_as_single_symbol(&self) -> Option<&SymbolRef<'a>> {
        self.exception
            .is_none()
            .then(|| self.factor.try_as_single_symbol())
            .flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub factor: Factor,
    pub exception: Option<Exception>,
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
