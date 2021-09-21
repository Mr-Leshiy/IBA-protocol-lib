extern crate proc_macro;
use parse::ScriptDefinition;
use proc_macro::TokenStream;
use quote::quote;

mod parse;

#[proc_macro]
pub fn script_eval(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    script_eval_impl(&ast)
}

fn script_eval_impl(ast: &ScriptDefinition) -> TokenStream {
    let gen = quote! {};
    gen.into()
}
