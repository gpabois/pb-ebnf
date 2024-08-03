use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parenthesized,
    parse::{discouraged::Speculative, Parse},
    token::{Brace, Bracket, Paren},
    Token,
};

fn into_slice<T>(iter: impl Iterator<Item = T>) -> TokenStream
where
    T: ToTokens,
{
    let a = iter
        .map(|t| t.to_token_stream())
        .reduce(|a, b| quote! {#a, #b})
        .unwrap_or(quote! {});

    quote! {&[#a]}
}

// syntax = rule {rule};
pub struct Syntax(Vec<Rule>);
impl Parse for Syntax {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ls = Vec::<Rule>::default();

        while !input.is_empty() {
            ls.push(
                input
                    .parse::<Rule>()
                    .map_err(|_| syn::Error::new(Span::call_site(), "expecting a rule"))?,
            );
        }

        Ok(Self(ls))
    }
}
impl ToTokens for Syntax {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let rules = into_slice(self.0.iter());
        tokens.extend(quote! {
            ::pb_ebnf::Syntax::new(#rules)
        })
    }
}

// rule = meta identifier, '=', definitions list, ';';
pub struct Rule {
    lhs: Symbol,
    rhs: DefinitionsList,
}
impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = input
            .parse::<Symbol>()
            .map_err(|_| syn::Error::new(Span::call_site(), "expecting a symbol as rule's lhs"))?;

        input
            .parse::<Token![=]>()
            .map_err(|_| syn::Error::new(Span::call_site(), "expecting a = after rule's lhs"))?;

        let rhs = input.parse::<DefinitionsList>().map_err(|_| {
            syn::Error::new(
                Span::call_site(),
                "expecting a definitions list as rule's rhs",
            )
        })?;

        input
            .parse::<Token![;]>()
            .map_err(|_| syn::Error::new(Span::call_site(), "expecting a ;"))?;

        Ok(Self { lhs, rhs })
    }
}
impl ToTokens for Rule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let lhs = &self.lhs;
        let rhs = &self.rhs;

        tokens.extend(quote! {
            ::pb_ebnf::Rule::new(#lhs, #rhs)
        })
    }
}

// definitions list = single definition, {'|' single definition};
pub struct DefinitionsList(Vec<SingleDefinition>);
impl Parse for DefinitionsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut list = vec![input.parse::<SingleDefinition>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<Token![|]>().is_err() {
                break;
            }

            if let Ok(def) = fork.parse::<SingleDefinition>() {
                list.push(def);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl ToTokens for DefinitionsList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let defs = into_slice(self.0.iter());
        tokens.extend(quote! {
            ::pb_ebnf::DefinitionsList::new(#defs)
        })
    }
}

// single definition = term, {',' term};
pub struct SingleDefinition(Vec<Term>);
impl Parse for SingleDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut list = vec![input.parse::<Term>()?];

        loop {
            let fork = input.fork();

            if fork.parse::<Token![,]>().is_err() {
                break;
            }

            if let Ok(term) = fork.parse::<Term>() {
                list.push(term);
                input.advance_to(&fork);
            } else {
                break;
            }
        }

        Ok(Self(list))
    }
}
impl ToTokens for SingleDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let terms = into_slice(self.0.iter());
        tokens.extend(quote! {
            ::pb_ebnf::SingleDefinition::new(#terms)
        })
    }
}

// term = factor, ["-", exception];
pub struct Term {
    factor: Factor,
    exception: Option<Exception>,
}
impl Parse for Term {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let factor = input.parse::<Factor>()?;
        let exception = if input.peek(Token![-]) {
            input.parse::<Exception>().map(Some)?
        } else {
            None
        };

        Ok(Self { factor, exception })
    }
}
impl ToTokens for Term {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let factor = &self.factor;
        let exception = self
            .exception
            .as_ref()
            .map(|exception| quote! {Some(#exception)})
            .unwrap_or(quote! {None});

        tokens.extend(quote! {::pb_ebnf::Term::new(#factor, #exception)});
    }
}

// exception = factor;
pub struct Exception(Factor);
impl Parse for Exception {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Factor>().map(Self)
    }
}
impl ToTokens for Exception {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let factor = &self.0;
        tokens.extend(quote! {
            ::pb_ebnf::Exception::new(#factor)
        })
    }
}

// factor = [integer '*'], primary;
pub struct Factor {
    repetition: Option<u32>,
    primary: Primary,
}
impl Parse for Factor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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
impl ToTokens for Factor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let repetition = self
            .repetition
            .map(|rep| quote! {Some(#rep)})
            .unwrap_or(quote! {None});
        let primary = &self.primary;

        tokens.extend(quote! {
            ::pb_ebnf::Factor::new(#primary, #repetition)
        })
    }
}

pub enum Primary {
    Optional(OptionalSequence),
    Repeated(RepeatedSequence),
    Grouped(GroupedSequence),
    Symbol(Symbol),
    Literal(Literal),
    Empty,
}
impl Parse for Primary {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // [
        if input.peek(Bracket) {
            input.parse::<OptionalSequence>().map(Self::Optional)
        } else if input.peek(Brace) {
            input.parse::<RepeatedSequence>().map(Self::Repeated)
        } else if input.peek(Paren) {
            input.parse::<GroupedSequence>().map(Self::Grouped)
        } else if input.peek(syn::LitStr) {
            input.parse::<Literal>().map(Self::Literal)
        } else if input.peek(syn::Ident) {
            input.parse::<Symbol>().map(Self::Symbol)
        } else {
            Ok(Self::Empty)
        }
    }
}
impl ToTokens for Primary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &self {
            Primary::Optional(a) => quote! {::pb_ebnf::Primary::Optional(#a)},
            Primary::Repeated(a) => quote! {::pb_ebnf::Primary::Repeated(#a)},
            Primary::Grouped(a) => quote! {::pb_ebnf::Primary::Grouped(#a)},
            Primary::Symbol(a) => quote! {::pb_ebnf::Primary::Symbol(#a)},
            Primary::Literal(a) => quote! {::pb_ebnf::Primary::Literal(#a)},
            Primary::Empty => quote! {::pb_ebnf::Primary::Empty},
        })
    }
}

// optional = '[', definitions list, ']';
pub struct OptionalSequence(DefinitionsList);
impl Parse for OptionalSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl ToTokens for OptionalSequence {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::OptionalSequence::new(#a)})
    }
}

// repeated = '{', definitions list , '}';
pub struct RepeatedSequence(DefinitionsList);
impl Parse for RepeatedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl ToTokens for RepeatedSequence {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::RepeatedSequence::new(#a)})
    }
}
// grouped = '(', definitions list, ')';
pub struct GroupedSequence(DefinitionsList);
impl Parse for GroupedSequence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let seq = content.parse::<DefinitionsList>()?;
        Ok(Self(seq))
    }
}
impl ToTokens for GroupedSequence {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.0;
        tokens.extend(quote! {::pb_ebnf::GroupedSequence::new(#a)})
    }
}

pub struct Symbol(String);
impl Parse for Symbol {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let mut parts = Vec::<String>::default();
            while !input.peek(Token![>]) {
                parts.push(input.parse::<syn::Ident>()?.to_string());
            }
            input.parse::<Token![>]>()?;
            Ok(Self(parts.join(" ")))
        } else {
            input.parse::<syn::Ident>().map(|id| Self(id.to_string()))
        }
    }
}
impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = &self.0;
        tokens.extend(quote! {
            ::pb_ebnf::Symbol::new(#value)
        })
    }
}

pub struct Literal(String);
impl Parse for Literal {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::LitStr>().map(|s| Self(s.value()))
    }
}
impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = &self.0;
        tokens.extend(quote! {
            ::pb_ebnf::Literal::new(#value)
        })
    }
}
