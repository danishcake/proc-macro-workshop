use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Construct an Ident with the name of the CommandBuilder
    // The quote macro suppors quoting these as
    // #ident_name
    let command_builder_name = format_ident!("{}Builder", input.ident);

    // Generate the list of optional fields
    let named_fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let field_options = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        quote! {
            #field_name: Option<#field_type>
        }
    });

    // Generate the list of field initialisers
    let field_initialisers = named_fields.iter().map(|f| {
        let field_name = &f.ident;

        quote! {
            #field_name: None
        }
    });

    // Generate the list of setters
    let setters = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        quote! {
            fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    });

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

        impl #command_builder_name {
            #(#setters)*
        }
    };

    output.into()
}
