use proc_macro::TokenStream;
use syn::{parse_macro_input, parse::{Parse, ParseStream, ParseBuffer}, Token, LitInt, Error, Ident, braced};
use quote::quote;
use std::{result::Result, str::ParseBoolError};
use std::ops;

struct Sequence {
    iteration_var: syn::Ident,
    lower_bound: LitInt,
    upper_bound: LitInt,
    body: TokenStream
}

impl Parse for Sequence {

    // Parses the input to the seq macro
    // This will consist of a variable, range and a block
    // seq!(N in 0..8 {
    //      //nothing
    // });
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let iteration_var: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let lower_bound: LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let upper_bound: LitInt = input.parse()?;
        let body_buffer;
        braced!(body_buffer in input);
        let body = quote!(body_buffer);

        Ok(Sequence {
            iteration_var,
            lower_bound,
            upper_bound,
            body: body.into()
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Sequence);

    TokenStream::new()
}
