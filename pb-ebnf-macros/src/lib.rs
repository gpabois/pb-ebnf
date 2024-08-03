use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod ebnf;

#[proc_macro]
pub fn ebnf(input: TokenStream) -> TokenStream {
    let syntax: ebnf::Syntax = parse_macro_input!(input);
    quote! {#syntax}.into()
}
