use super::argument::*;
use super::opcode::*;
use parity_scale_codec::{Decode, Encode, Input};

#[derive(Default)]
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
                    let arg1: u64 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    let arg2: u64 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push(Argument::new().set_value_chain(arg1 + arg2));
                }
                //
                code if code == OP_SUB => {
                    let arg1: u64 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    let arg2: u64 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    args_stack.push(Argument::new().set_value_chain(arg2 - arg1));
                }
                //
                code if code == OP_EQL => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    args_stack.push(Argument::new().set_value_chain(arg1 == arg2));
                }
                //
                code if code == OP_NQL => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    args_stack.push(Argument::new().set_value_chain(arg1 != arg2));
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
        script.push_argument(&Argument::new().set_value_chain(6 as u64));

        script.push_argument(&Argument::new().set_value_chain(5 as u64));
        script.push_op_code(&OP_ADD);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<u64>(),
            Ok(11)
        );
    }

    #[test]
    fn op_sub_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(5 as u64));
        script.push_op_code(&OP_SUB);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<u64>(),
            Ok(1)
        );
    }

    #[test]
    fn op_eql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_op_code(&OP_EQL);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(5 as u64));
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_op_code(&OP_EQL);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(false)
        );
    }

    #[test]
    fn op_nql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(5 as u64));
        script.push_op_code(&OP_NQL);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_op_code(&OP_NQL);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(false)
        );
    }

    #[test]
    fn script_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(8 as u64));
        script.push_op_code(&OP_ADD);
        script.push_argument(&Argument::new().set_value_chain(12 as u64));
        script.push_op_code(&OP_SUB);
        script.push_argument(&Argument::new().set_value_chain(2 as u64));
        script.push_op_code(&OP_EQL);

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );
    }
}
