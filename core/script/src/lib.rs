use argument::*;
use opcode::*;
use parity_scale_codec::{Decode, Encode};

pub mod argument;
pub mod opcode;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Script {
    data: Vec<u8>,
}

impl Script {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push_argument(&mut self, arg: &Argument) {
        self.data.append(&mut arg.to_script());
    }

    pub fn push_op_code<Op: OpCode>(&mut self) {
        self.data.append(&mut Op::CODE.encode());
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn push_argument_chain(mut self, arg: &Argument) -> Self {
        self.data.append(&mut arg.to_script());
        self
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn push_op_code_chain<Op: OpCode>(mut self) -> Self {
        self.data.append(&mut Op::CODE.encode());
        self
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Debug)]
pub enum ScriptError {
    UnknownOpCode(u32),
    InvalidArguments(OpCodeError),
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
}
