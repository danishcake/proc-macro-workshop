use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Construct an Ident with the name of the CommandBuilder
    // The quote macro suppors quoting these as
    // #ident_name
    let command_builder_name = format_ident!("{}Builder", input.ident);

    // Generate the list of optional fields
    let field_options = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;

                    quote! {
                        #field_name: Option<#field_type>
                    }
                }),
                _ => {
                    // We don't support tuple structs or tuple variants
                    unimplemented!()
                }
            }
        },
        _ => {
            // We don't support unions or enums
            unimplemented!()
        }
    };

    // Generate the list of field initialisers
    let field_initialisers = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => fields.named.iter().map(|f| {
                    let field_name = &f.ident;

                    quote! {
                        #field_name: None
                    }
                }),
                _ => {
                    // We don't support tuple structs or tuple variants
                    unimplemented!()
                }
            }
        },
        _ => {
            // We don't support unions or enums
            unimplemented!()
        }
    };

    // quote! allows us to write code, and have it transformed into a TokenStream
    // (albeit a proc_macro2::TokenStream, hence the .into())
    let output = quote! {
        pub struct #command_builder_name {
            #(#field_options),*
        }


        impl Command {
            pub fn builder() -> #command_builder_name {
                #command_builder_name {
                    #(#field_initialisers),*
                }
            }
        }
    };

    output.into()
}
