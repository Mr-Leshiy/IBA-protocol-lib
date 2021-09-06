use parity_scale_codec::{Decode, Encode, Input};

struct Script {
    data: Vec<u8>,
}

#[derive(Debug)]
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
static OP_EQL: OpCode = OpCode { code: 5 };
static OP_NQL: OpCode = OpCode { code: 6 };

impl Script {
    pub fn evaluate(&self) -> Result<Option<Vec<u8>>, ScriptError> {
        let mut data = self.data.as_slice();

        let mut args_stack = Vec::new();

        // while not end of the stream
        while data.remaining_len() != Ok(Some(0)) {
            let prev_data = data;

            // try to decode argument
            if let Ok(arg) = Vec::<u8>::decode(&mut data) {
                args_stack.push(arg);
                continue;
            }

            data = prev_data;
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

                    args_stack.push((arg2 - arg1).encode());
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
    fn script_add_test() {
        let mut data = Vec::new();
        data.append(&mut (5 as u64).encode().encode());
        data.append(&mut (6 as u64).encode().encode());
        data.append(&mut OP_ADD.encode());

        assert_eq!(
            u64::decode(&mut Script { data }.evaluate().unwrap().unwrap().as_ref()),
            Ok(11)
        );
    }

    #[test]
    fn script_sub_test() {
        let mut data = Vec::new();
        data.append(&mut (6 as u64).encode().encode());
        data.append(&mut (5 as u64).encode().encode());
        data.append(&mut OP_SUB.encode());

        assert_eq!(
            u64::decode(&mut Script { data }.evaluate().unwrap().unwrap().as_ref()),
            Ok(1)
        );
    }
}
