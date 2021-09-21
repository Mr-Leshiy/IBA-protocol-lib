extern crate proc_macro;
use core::panic;
use parse::{OpCodeDefinition, ScriptDefinition};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod parse;

#[proc_macro]
pub fn eval(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    eval_impl(ast).into()
}

fn decl_op_codes(op_codes: impl Iterator<Item = OpCodeDefinition>) -> TokenStream2 {
    op_codes.fold(TokenStream2::default(), |combined, op_code| {
        let name = op_code.name;
        quote! {#combined
             #name::CODE => {
                let res = #name::handler(());
                args_stack.push(script::argument::Argument::new().set_value_chain(res));
             }
        }
    })
}

fn eval_impl(script: ScriptDefinition) -> TokenStream2 {
    let op_codes = decl_op_codes(script.op_codes.into_iter());
    panic!("op_codes: {}", op_codes.to_string());
    quote! {}
}
