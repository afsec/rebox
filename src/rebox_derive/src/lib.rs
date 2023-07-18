use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DbEntity)]
pub fn derive_db_prefix(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl DbPrefix for #name {}
    };
    gen.into()
}

#[proc_macro_derive(DbDriver)]
pub fn derive_db_driver(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Driver for #name {}
    };
    gen.into()
}
