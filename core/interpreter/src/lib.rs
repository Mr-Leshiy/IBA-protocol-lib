extern crate proc_macro;

use parse::{OpCodeDefinition, ScriptDefinition};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod parse;

#[proc_macro]
pub fn interpret(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    interpret_impl(ast).into()
}

fn decl_op_codes(op_codes: impl Iterator<Item = OpCodeDefinition>) -> TokenStream2 {
    op_codes.fold(TokenStream2::default(), |combined, op_code| {
        let name = op_code.name;
        quote! {
            #combined
            #name::CODE => {
                let args = <#name as script::opcode::OpCode>::Args::decode_arguments(&mut args_stack)
                .map_err(script::ScriptError::InvalidArguments)?;

                #name::handler(args).encode_arguments(&mut args_stack);
            }
        }
    })
}

fn interpret_impl(script: ScriptDefinition) -> TokenStream2 {
    let op_codes_decl = decl_op_codes(script.op_codes.into_iter());

    let script_decl = script.name;

    let interpret_decl = quote! {
        {
            let f = || {
                use parity_scale_codec::{Decode, Encode, Input};
                use script::opcode::*;
                use script::*;

                let data = #script_decl.get_data();
                let mut data = data.as_slice();

                let mut args_stack = Vec::new();

                // // while not end of the stream
                while data.remaining_len() != Ok(Some(0)) {
                    match u32::decode(&mut data).unwrap() {
                        OpPush::CODE => {
                            let arg = script::ScriptValue::decode(&mut data).unwrap();

                            OpPush::handler(arg).encode_arguments(&mut args_stack);
                        }
                        OpEql::CODE => {
                            let args = <OpEql as OpCode>::Args::decode_arguments(&mut args_stack)
                                .map_err(ScriptError::InvalidArguments)?;

                            OpEql::handler(args).encode_arguments(&mut args_stack);
                        }
                        OpNql::CODE => {
                            let args = <OpNql as OpCode>::Args::decode_arguments(&mut args_stack)
                                .map_err(ScriptError::InvalidArguments)?;

                            OpNql::handler(args).encode_arguments(&mut args_stack);
                        }
                        script::opcode::OpAdd::CODE => {
                            let args = <OpAdd as OpCode>::Args::decode_arguments(&mut args_stack)
                                .map_err(ScriptError::InvalidArguments)?;

                            OpAdd::handler(args).encode_arguments(&mut args_stack);
                        }
                        OpSub::CODE => {
                            let args = <OpSub as OpCode>::Args::decode_arguments(&mut args_stack)
                                .map_err(ScriptError::InvalidArguments)?;

                            OpSub::handler(args).encode_arguments(&mut args_stack);
                        }

                        #op_codes_decl

                        code => {
                            return Err(ScriptError::UnknownOpCode(code));
                        }
                    }
                }
                Ok(args_stack.pop())
            };
            f()
        }
    };
    interpret_decl
}
