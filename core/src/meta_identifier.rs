use std::ops::Deref;

pub trait MetaIdentifier: Deref<Target = str> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaIdentifierRef<'a>(pub(crate) &'a str);
impl<'a> MetaIdentifierRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}

impl MetaIdentifier for MetaIdentifierRef<'_> {}

impl Deref for MetaIdentifierRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnedMetaIdentifier(String);

impl OwnedMetaIdentifier {
    pub fn new<S: ToString>(id: S) -> Self {
        Self(id.to_string())
    }

    pub fn borrow(&self) -> MetaIdentifierRef<'_> {
        MetaIdentifierRef(self)
    }
}

impl MetaIdentifier for OwnedMetaIdentifier {}

impl Deref for OwnedMetaIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OwnedMetaIdentifier {
    pub fn is_beginning_of_symbol(input: &syn::parse::ParseStream) -> bool {
        use syn::Token;
        [
            input.peek(Token![<]),
            input.peek(Token![self]),
            input.peek(Token![Self]),
            input.peek(Token![as]),
            input.peek(Token![default]),
            input.peek(Token![where]),
            input.peek(Token![type]),
            input.peek(Token![match]),
            input.peek(syn::Ident),
        ]
        .into_iter()
        .any(|x| x)
    }
}

impl syn::parse::Parse for OwnedMetaIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let mut parts = Vec::<String>::default();
            while !input.peek(Token![>]) {
                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    parts.push("-".to_owned());
                } else if input.peek(Token![self]) {
                    input.parse::<Token![self]>()?;
                    parts.push("self".to_owned());
                } else if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    parts.push("as".to_owned());
                } else if input.peek(Token![where]) {
                    input.parse::<Token![where]>()?;
                    parts.push("as".to_owned());
                } else if input.peek(Token![default]) {
                    input.parse::<Token![default]>()?;
                    parts.push("default".to_owned());
                } else if input.peek(Token![type]) {
                    input.parse::<Token![type]>()?;
                    parts.push("type".to_owned());
                } else if input.peek(Token![match]) {
                    input.parse::<Token![match]>()?;
                    parts.push("match".to_owned());
                } else if input.peek(Token![break]) {
                    input.parse::<Token![break]>()?;
                    parts.push("break".to_owned());
                } else if input.peek(Token![const]) {
                    input.parse::<Token![const]>()?;
                    parts.push("const".to_owned());
                } else if input.peek(Token![continue]) {
                    input.parse::<Token![continue]>()?;
                    parts.push("continue".to_owned());
                } else if input.peek(Token![static]) {
                    input.parse::<Token![static]>()?;
                    parts.push("continue".to_owned());
                } else if input.peek(Token![in]) {
                    input.parse::<Token![in]>()?;
                    parts.push("in".to_owned());
                } else if input.peek(Token![if]) {
                    input.parse::<Token![if]>()?;
                    parts.push("if".to_owned());
                } else if input.peek(Token![else]) {
                    input.parse::<Token![else]>()?;
                    parts.push("else".to_owned());
                } else if input.peek(Token![ref]) {
                    input.parse::<Token![ref]>()?;
                    parts.push("ref".to_owned());
                } else if input.peek(Token![return]) {
                    input.parse::<Token![return]>()?;
                    parts.push("return".to_owned());
                } else {
                    parts.push(input.parse::<syn::Ident>()?.to_string());
                }
            }
            input.parse::<Token![>]>()?;
            Ok(Self(parts.join(" ")))
        } else {
            input.parse::<syn::Ident>().map(|id| Self(id.to_string()))
        }
    }
}
impl quote::ToTokens for OwnedMetaIdentifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let value = &self.0;
        tokens.extend(quote! {
            ::pb_ebnf::MetaIdentifierRef::new(#value)
        })
    }
}
