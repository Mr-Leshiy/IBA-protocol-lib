use parity_scale_codec::{Decode, Encode, Input};
use super::opcode::*;
struct Script {
    data: Vec<u8>,
}

#[derive(Debug)]
enum ScriptError {
    UnknownOpCode(OpCode),
    InvalidArgumentAmount,
    UnexepectedArgumentType,
}

#[derive(Decode, Encode, Debug)]
struct Argument {
    // encoded
    data: Vec<u8>,
}

impl Argument {
    pub fn to_script(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut OP_PUSH.encode());
        data.append(&mut self.encode());
        data
    }
}

impl Script {
    pub fn evaluate(&self) -> Result<Option<Argument>, ScriptError> {
        let mut data = self.data.as_slice();

        let mut args_stack = Vec::new();

        // while not end of the stream
        while data.remaining_len() != Ok(Some(0)) {
            match OpCode::decode(&mut data).unwrap() {
                code if code == OP_PUSH => {
                    let arg = Argument::decode(&mut data).unwrap();
                    args_stack.push(arg);
                }
                //
                code if code == OP_ADD => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .data
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .data
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push(Argument {
                        data: (arg1 + arg2).encode(),
                    });
                }
                //
                code if code == OP_SUB => {
                    let arg1 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .data
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = u64::decode(
                        &mut args_stack
                            .pop()
                            .ok_or(ScriptError::InvalidArgumentAmount)?
                            .data
                            .as_ref(),
                    )
                    .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push(Argument {
                        data: (arg2 - arg1).encode(),
                    });
                }
                //
                code if code == OP_EQL => {
                    let arg1 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .data;
                    let arg2 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .data;

                    args_stack.push(Argument {
                        data: (arg1 == arg2).encode(),
                    });
                }
                //
                code if code == OP_NQL => {
                    let arg1 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .data;
                    let arg2 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .data;

                    args_stack.push(Argument {
                        data: (arg1 != arg2).encode(),
                    });
                }
                code => return Err(ScriptError::UnknownOpCode(code)),
            }
        }

        Ok(args_stack.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_add_test() {
        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_ADD.encode());

        assert_eq!(
            u64::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(11)
        );
    }

    #[test]
    fn script_sub_test() {
        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_SUB.encode());

        assert_eq!(
            u64::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(1)
        );
    }

    #[test]
    fn script_eql_test() {
        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_EQL.encode());

        assert_eq!(
            bool::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(true)
        );

        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_EQL.encode());

        assert_eq!(
            bool::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(false)
        );
    }

    #[test]
    fn script_nql_test() {
        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (6 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_NQL.encode());

        assert_eq!(
            bool::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(true)
        );

        let mut data = Vec::new();
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(
            &mut Argument {
                data: (5 as u64).encode(),
            }
            .to_script(),
        );
        data.append(&mut OP_NQL.encode());

        assert_eq!(
            bool::decode(&mut Script { data }.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(false)
        );
    }
}
