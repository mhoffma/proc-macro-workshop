use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());
    //eprintln!("{:#?}", ast);

    let expanded = quote! {
    pub struct #bident {
    }
    impl #name {
        pub fn builder() -> #bident { #bident {} }
    }
    };
    expanded.into()
}
