use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralRef<'a>(pub(crate) &'a str);
impl<'a> LiteralRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}
impl Deref for LiteralRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedLiteral(String);

impl OwnedLiteral {
    pub fn new<S: ToString>(value: S) -> Self {
        Self(value.to_string())
    }

    pub fn borrow(&self) -> LiteralRef<'_> {
        LiteralRef(self)
    }
}

impl Deref for OwnedLiteral {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for OwnedLiteral {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitChar) {
            input
                .parse::<syn::LitChar>()
                .map(|s| Self(s.value().to_string()))
        } else {
            input.parse::<syn::LitStr>().map(|s| Self(s.value()))
        }
    }
}

impl quote::ToTokens for OwnedLiteral {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let value = &self.0;
        tokens.extend(quote! {
            ::pb_ebnf::LiteralRef::new(#value)
        })
    }
}
