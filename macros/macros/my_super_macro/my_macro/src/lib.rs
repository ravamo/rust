extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let name = &input.ident;
    let gen = quote! {
        impl #name {
            pub fn hello() -> String {
                format!("Hello, {}!", stringify!(#name))
            }
        }
    };

    // Hand the output tokens back to the compiler
    gen.into()
}
