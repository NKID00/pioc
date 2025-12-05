use std::path::PathBuf;

use pioc_asm::assemble_to_words;

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error, set_dummy};
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro_error]
#[proc_macro]
pub fn pioc_inner(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let literal = parse_macro_input!(input as LitStr);
    let asm = literal.value();
    let words = assemble_to_words(asm).unwrap_or_else(|e| abort!(literal, e.to_string()));
    quote! { [#(#words),*] }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn pioc_include_inner(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let literal = parse_macro_input!(input as LitStr);
    let path = PathBuf::from(literal.value());
    let asm = std::fs::read_to_string(path).unwrap_or_else(|e| abort!(literal, e.to_string()));
    let words = assemble_to_words(asm).unwrap_or_else(|e| abort!(literal, e.to_string()));
    quote! { [#(#words),*] }.into()
}
