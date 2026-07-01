extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod register;
mod register_field;

#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let def = parse_macro_input!(input as register::RegisterDef);
    match def.expand() {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(RegisterField)]
pub fn derive_register_field(input: TokenStream) -> TokenStream {
    let def = parse_macro_input!(input as register_field::RegisterFieldDef);
    match def.expand() {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
