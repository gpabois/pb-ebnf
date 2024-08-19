mod itertools;

pub mod definitions_list;
pub mod exception;
pub mod factor;
pub mod grouped;
pub mod literal;
pub mod meta_identifier;
pub mod optional;
pub mod prelude;
pub mod primary;
pub mod repeated;
pub mod rule;
pub mod single_definition;
pub mod symbol;
pub mod syntax;
pub mod term;

pub use definitions_list::*;
pub use exception::*;
pub use factor::*;
pub use grouped::*;
pub use literal::*;
pub use meta_identifier::*;
pub use optional::*;
pub use primary::*;
pub use repeated::*;
pub use rule::*;
pub use single_definition::*;
pub use symbol::*;
pub use syntax::*;
pub use term::*;

pub type StaticSyntax = SyntaxRef<'static>;

fn into_slice<T>(iter: impl Iterator<Item = T>) -> proc_macro2::TokenStream
where
    T: quote::ToTokens,
{
    let a = iter
        .map(|t| t.to_token_stream())
        .reduce(|a, b| quote::quote! {#a, #b})
        .unwrap_or(quote::quote! {});

    quote::quote! {&[#a]}
}
