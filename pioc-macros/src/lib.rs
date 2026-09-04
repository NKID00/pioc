use std::path::PathBuf;

use pioc_asm::{SymTab, assemble_parsed, assemble_parsed_with_symbols, parse};

use proc_macro::TokenStream;
use proc_macro_error::{abort, emit_error, proc_macro_error, set_dummy};
use quote::quote;
use syn::{LitStr, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

#[proc_macro_error]
#[proc_macro]
pub fn pioc_inner(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let lit = parse_macro_input!(input as LitStr);
    let asm = lit.value();
    let bytes = assemble_reporting_errors(lit, asm);
    quote! { [#(#bytes),*] }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn pioc_inner_with_exports(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let mut lits = Punctuated::<LitStr, Token![,]>::parse_terminated
        .parse(input)
        .unwrap()
        .into_iter();
    let asm_lit = lits.next().unwrap();
    let asm = asm_lit.value();
    let exports_lits: Vec<LitStr> = lits.collect();
    let (bytes, exports) = assemble_reporting_errors_with_exports(asm_lit, asm, exports_lits);
    quote! { ([#(#bytes),*], #(#exports),*) }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn pioc_include_inner(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let lit = parse_macro_input!(input as LitStr);
    let path = PathBuf::from(lit.value());
    let asm = std::fs::read_to_string(path).unwrap_or_else(|e| abort!(lit, e.to_string()));
    let stmts = parse(asm).unwrap_or_else(|e| abort!(lit, e.to_string()));
    let insts = assemble_parsed(&stmts).unwrap_or_else(|e| abort!(lit, e.to_string()));
    let bytes: Vec<u8> = insts.into_iter().flat_map(|inst| inst.to_bytes()).collect();
    quote! { [#(#bytes),*] }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn pioc_include_inner_with_exports(input: TokenStream) -> TokenStream {
    set_dummy(quote! { panic!() });
    let mut lits = Punctuated::<LitStr, Token![,]>::parse_terminated
        .parse(input)
        .unwrap()
        .into_iter();
    let asm_lit = lits.next().unwrap();
    let path = PathBuf::from(asm_lit.value());
    let asm = std::fs::read_to_string(path).unwrap_or_else(|e| abort!(asm_lit, e.to_string()));
    let exports_lits: Vec<LitStr> = lits.collect();
    let (bytes, exports) = assemble_reporting_errors_with_exports(asm_lit, asm, exports_lits);
    quote! { ([#(#bytes),*], #(#exports),*) }.into()
}

fn assemble_reporting_errors(lit: LitStr, asm: String) -> Vec<u8> {
    let stmts = parse(asm).unwrap_or_else(|e| abort!(lit, e.to_string()));
    let insts = assemble_parsed(&stmts).unwrap_or_else(|e| abort!(lit, e.to_string()));
    insts.into_iter().flat_map(|inst| inst.to_bytes()).collect()
}

fn assemble_reporting_errors_with_exports(
    asm_lit: LitStr,
    asm: String,
    exports_lits: Vec<LitStr>,
) -> (Vec<u8>, Vec<i32>) {
    let stmts = parse(asm).unwrap_or_else(|e| abort!(asm_lit, e.to_string()));
    let (sym, insts) = assemble_parsed_with_symbols(&SymTab::default(), &stmts)
        .unwrap_or_else(|e| abort!(asm_lit, e.to_string()));
    let mut exports = vec![];
    for export_lit in exports_lits {
        let export = export_lit.value();
        match sym.get(&export) {
            Some(v) => exports.push(*v),
            None => emit_error!(export_lit, "undefined symbol"),
        }
    }
    let bytes: Vec<u8> = insts.into_iter().flat_map(|inst| inst.to_bytes()).collect();
    (bytes, exports)
}
