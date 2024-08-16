use pb_ebnf_core::syntax::Syntax;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn ebnf(input: TokenStream) -> TokenStream {
    let syntax: Syntax = parse_macro_input!(input);
    quote! {#syntax}.into()
}
