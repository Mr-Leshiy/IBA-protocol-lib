use parity_scale_codec::{Decode, Encode, Input};

struct Script {
    data: Vec<u8>,
}

enum ScriptError {
    UnknownOpCode(OpCode),
    InvalidArgumentAmount,
    UnexepectedArgumentType,
}

#[derive(Decode, Encode, PartialEq, Debug)]
struct OpCode {
    code: u32,
}

static OP_ADD: OpCode = OpCode { code: 1 };
static OP_SUB: OpCode = OpCode { code: 2 };
static OP_MUL: OpCode = OpCode { code: 3 };
static OP_DIV: OpCode = OpCode { code: 4 };
static OP_EQL: OpCode = OpCode { code: 5 };
static OP_NQL: OpCode = OpCode { code: 6 };

impl Script {
    pub fn evaluate(&self) -> Result<Option<Vec<u8>>, ScriptError> {
        let mut data = self.data.as_slice();

        let mut args_stack = Vec::new();

        // while not end of the stream
        while data.remaining_len() != Ok(Some(0)) {
            // try to decode argument
            if let Ok(arg) = Vec::<u8>::decode(&mut data) {
                args_stack.push(arg);
                continue;
            }

            match OpCode::decode(&mut data).unwrap() {
                code if code == OP_ADD => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push((arg1 + arg2).encode());
                }
                code if code == OP_SUB => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push((arg1 - arg2).encode());
                }
                code if code == OP_MUL => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push((arg1 * arg2).encode());
                }
                code if code == OP_DIV => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push((arg1 / arg2).encode());
                }
                code if code == OP_EQL => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;
                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    args_stack.push((arg1 == arg2).encode());
                }
                code if code == OP_NQL => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;
                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    args_stack.push((arg1 != arg2).encode());
                }
                code => return Err(ScriptError::UnknownOpCode(code)),
            }
        }

        Ok(args_stack.pop())
    }
}

mod tests {
    use super::*;

    #[test]
    fn script_evaluate_test() {
        let code1 = OpCode { code: 10 };
        let code2 = OpCode { code: 12 };
        let code3 = OpCode { code: 13 };

        let mut data = Vec::new();
        data.append(&mut code1.encode());
        data.append(&mut code2.encode());
        data.append(&mut code3.encode());

        let script = Script { data: data.clone() };

        assert!(script.evaluate().is_ok());
    }
}
