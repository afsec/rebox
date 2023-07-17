use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DbEntity)]
pub fn derive_prefix(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl DbPrefix for #name {}
    };
    gen.into()
}
