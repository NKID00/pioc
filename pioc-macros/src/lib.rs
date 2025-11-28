use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn pioc_inner(input: TokenStream) -> TokenStream {
    let mut words: Vec<u16> = Vec::with_capacity(2048);
    words.push(0);
    words.push(0);
    quote! {
        [
            #(#words),*
        ]
    }
    .into()
}
