use super::opcode::*;
use parity_scale_codec::{Decode, Encode, Input};

struct Script {
    data: Vec<u8>,
}

impl Script {
    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_argument(&mut self, arg: &Argument) {
        self.data.append(&mut arg.to_script());
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_op_code(&mut self, op_code: &OpCode) {
        self.data.append(&mut op_code.encode());
    }
}

#[derive(Debug)]
enum ScriptError {
    UnknownOpCode(OpCode),
    InvalidArgumentAmount,
    UnexepectedArgumentType,
}

#[derive(Decode, Encode, Debug)]
struct Argument {
    data: Vec<u8>,
}

impl Argument {
    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn to_script(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut OP_PUSH.encode());
        data.append(&mut self.encode());
        data
    }
}

impl Script {
    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
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
    fn op_add_test() {
        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });

        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_op_code(&OP_ADD);

        assert_eq!(
            u64::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(11)
        );
    }

    #[test]
    fn op_sub_test() {
        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_op_code(&OP_SUB);

        assert_eq!(
            u64::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(1)
        );
    }

    #[test]
    fn op_eql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_op_code(&OP_EQL);

        assert_eq!(
            bool::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_op_code(&OP_EQL);

        assert_eq!(
            bool::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(false)
        );
    }

    #[test]
    fn op_nql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_op_code(&OP_NQL);

        assert_eq!(
            bool::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (5 as u64).encode(),
        });
        script.push_op_code(&OP_NQL);

        assert_eq!(
            bool::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(false)
        );
    }

    #[test]
    fn script_test() {
        let mut script = Script::new();
        script.push_argument(&Argument {
            data: (6 as u64).encode(),
        });
        script.push_argument(&Argument {
            data: (8 as u64).encode(),
        });
        script.push_op_code(&OP_ADD);
        script.push_argument(&Argument {
            data: (12 as u64).encode(),
        });
        script.push_op_code(&OP_SUB);
        script.push_argument(&Argument {
            data: (2 as u64).encode(),
        });
        script.push_op_code(&OP_EQL);

        assert_eq!(
            bool::decode(&mut script.evaluate().unwrap().unwrap().data.as_ref()),
            Ok(true)
        );
    }
}
