use argument::*;
use opcode::*;
use parity_scale_codec::{Decode, Encode, Input};

pub mod argument;
pub mod opcode;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Script {
    data: Vec<u8>,
}

impl Script {
    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_argument(&mut self, arg: &Argument) {
        self.data.append(&mut arg.to_script());
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_op_code<Op: OpCode>(&mut self) {
        self.data.append(&mut Op::CODE.encode());
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_argument_chain(mut self, arg: &Argument) -> Self {
        self.data.append(&mut arg.to_script());
        self
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    fn push_op_code_chain<Op: OpCode>(mut self) -> Self {
        self.data.append(&mut Op::CODE.encode());
        self
    }
}

#[derive(Debug)]
pub enum ScriptError {
    UnknownOpCode(u32),
    InvalidArgumentAmount,
    UnexepectedArgumentType,
}

impl Script {
    pub fn evaluate(&self) -> Result<Option<Argument>, ScriptError> {
        let mut data = self.data.as_slice();

        let mut args_stack = Vec::new();

        // while not end of the stream
        while data.remaining_len() != Ok(Some(0)) {
            match u32::decode(&mut data).unwrap() {
                OpPush::CODE => {
                    let arg = Argument::decode(&mut data).unwrap();
                    args_stack.push(OpPush::handler(arg));
                }
                OpEql::CODE => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;
                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    let res = OpEql::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
                }
                OpNql::CODE => {
                    let arg1 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;
                    let arg2 = args_stack.pop().ok_or(ScriptError::InvalidArgumentAmount)?;

                    let res = OpNql::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
                }
                OpAdd::CODE => {
                    let arg1 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    let res = OpAdd::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
                }
                OpSub::CODE => {
                    let arg1 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;
                    let arg2 = args_stack
                        .pop()
                        .ok_or(ScriptError::InvalidArgumentAmount)?
                        .get_value()
                        .map_err(|_| ScriptError::UnexepectedArgumentType)?;

                    let res = OpSub::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
                }
                code => return Err(ScriptError::UnknownOpCode(code)),
            }
        }

        Ok(args_stack.pop())
    }
}

pub mod tests {
    use super::*;

    pub fn default_script() -> Script {
        Script::new()
            .push_argument_chain(&Argument::new().set_value_chain(1_u64))
            .push_argument_chain(&Argument::new().set_value_chain(2_u64))
            .push_argument_chain(&Argument::new().set_value_chain(3_u64))
            .push_argument_chain(&Argument::new().set_value_chain(4_u64))
    }

    #[test]
    fn op_add_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));

        script.push_argument(&Argument::new().set_value_chain(5_u64));
        script.push_op_code::<OpAdd>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<u64>(),
            Ok(11)
        );
    }

    #[test]
    fn op_sub_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(5_u64));
        script.push_op_code::<OpSub>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<u64>(),
            Ok(1)
        );
    }

    #[test]
    fn op_eql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_op_code::<OpEql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(5_u64));
        script.push_op_code::<OpEql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(false)
        );
    }

    #[test]
    fn op_nql_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(5_u64));
        script.push_op_code::<OpNql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_op_code::<OpNql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(false)
        );
    }

    #[test]
    fn script_test() {
        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6_u64));
        script.push_argument(&Argument::new().set_value_chain(8_u64));
        script.push_op_code::<OpAdd>();
        script.push_argument(&Argument::new().set_value_chain(12_u64));
        script.push_op_code::<OpSub>();
        script.push_argument(&Argument::new().set_value_chain(2_u64));
        script.push_op_code::<OpEql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );
    }
}
