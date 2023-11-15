use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Block, Expr, Ident, LitInt, Token};

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
        //let e: Expr = input.parse()?;
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
    eprintln!("{:#?}", ast);
    let n = ast.name;
    let b = ast.body;
    let mut tt = TokenStream::new();
    for c in ast.start..ast.end {
        eprintln!("{}", c);
        tt = quote!(#tt  let #n = #c;  #b).into();
    }
    tt
}
