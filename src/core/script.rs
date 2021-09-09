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
    fn push_op_code<Op: OpCode>(&mut self) {
        self.data.append(&mut Op::CODE.encode());
    }
}

#[derive(Debug)]
enum ScriptError {
    UnknownOpCode(u32),
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
            match u32::decode(&mut data).unwrap() {
                OpPush::CODE => {
                    let arg = Argument::decode(&mut data).unwrap();
                    args_stack.push(OpPush::handler(arg));
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
                OpEql::CODE => {
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

                    let res = OpNql::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
                }
                OpNql::CODE => {
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

                    let res = OpNql::handler((arg1, arg2));
                    args_stack.push(Argument::new().set_value_chain(res));
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
        script.push_op_code::<OpAdd>();

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
        script.push_op_code::<OpSub>();

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
        script.push_op_code::<OpEql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(5 as u64));
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_op_code::<OpEql>();

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
        script.push_op_code::<OpNql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );

        let mut script = Script::new();
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_argument(&Argument::new().set_value_chain(6 as u64));
        script.push_op_code::<OpNql>();

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
        script.push_op_code::<OpAdd>();
        script.push_argument(&Argument::new().set_value_chain(12 as u64));
        script.push_op_code::<OpSub>();
        script.push_argument(&Argument::new().set_value_chain(2 as u64));
        script.push_op_code::<OpEql>();

        assert_eq!(
            script.evaluate().unwrap().unwrap().get_value::<bool>(),
            Ok(true)
        );
    }
}
