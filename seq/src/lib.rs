use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Block, Ident, LitInt, Token};

#[derive(Debug)]
struct Seq {
    name: Ident,
    start: i32,
    end: i32,
    body: Block,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let lstart: LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let lend: LitInt = input.parse()?;
        let body: Block = input.parse()?;
        let start: i32 = lstart.base10_parse()?;
        let end: i32 = lend.base10_parse()?;
        Ok(Seq {
            name,
            start,
            end,
            body,
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Seq);
    let _n = ast.name;
    let b = ast.body;
    let mut tt = proc_macro2::TokenStream::new();
    let rep = (ast.start..ast.end).map(|_c| quote!(#b;));
    tt.extend(rep);
    //eprintln!("{:#?}", tt);

    tt.into()
}
