use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Construct an Ident with the name of the CommandBuilder
    // The quote macro suppors quoting these as
    // #ident_name
    let command_builder_name = format_ident!("{}Builder", input.ident);

    // quote! allows us to write code, and have it transformed into a TokenStream
    // (albeit a proc_macro2::TokenStream, hence the .into())
    let output = quote! {
        pub struct #command_builder_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }


        impl Command {
            pub fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    output.into()
}
