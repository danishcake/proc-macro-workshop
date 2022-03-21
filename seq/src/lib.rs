use proc_macro::TokenStream;
use quote::quote;
use std::result::Result;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Ident, LitInt, Token,
};

struct Sequence {
    iteration_var: syn::Ident,
    lower_bound: LitInt,
    upper_bound: LitInt,
    body: TokenStream,
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
            body: body.into(),
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Sequence);

    let unrolled_loop = (input.lower_bound.base10_parse::<i32>().unwrap()
        ..input.upper_bound.base10_parse::<i32>().unwrap())
        .map(|_f| {
            // The input.body is a proc_macro::TokenStream. Quoting is a lazy way to convert
            // to a proc_macro2::TokenStream
            // TODO: Do this conversion properly so that it actually compiles
            quote! {
                #input.body
            }
        });

    let output = quote! {
        #(#unrolled_loop)*
    };

    eprintln!("TOKENS: {}", output);
    output.into()
}
