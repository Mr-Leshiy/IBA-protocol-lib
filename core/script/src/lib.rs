use opcode::*;
use parity_scale_codec::{Decode, Encode, Error, Input};

pub mod opcode;

#[derive(Decode, Encode, PartialEq, Clone, Default, Debug)]
pub struct ScriptValue {
    data: Vec<u8>,
}

impl ScriptValue {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn to_script(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut OpPush::CODE.encode());
        data.append(&mut self.encode());
        data
    }

    pub fn set_value<T: Encode>(&mut self, val: &T) {
        self.data = val.encode();
    }

    pub fn set_value_chain<T: Encode>(mut self, val: &T) -> Self {
        self.data = val.encode();
        self
    }

    pub fn get_value<T: Decode>(&self) -> Result<T, Error> {
        T::decode(&mut self.data.as_ref())
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Script {
    data: Vec<u8>,

    #[codec(skip)]
    index: usize,
}

#[derive(Debug)]
pub enum ScriptError {
    CannotDecodeOpCode,
    CannotDecodeValue,
    EndOfScript,
    UnknownOpCode(u32),
    InvalidArguments(OpCodeError),
}

impl Script {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            index: 0,
        }
    }

    pub fn push_value<T: Encode>(&mut self, val: &T) {
        let val = ScriptValue::new().set_value_chain(val);
        self.data.append(&mut val.to_script());
    }

    pub fn push_op_code<Op: OpCode>(&mut self) {
        self.data.append(&mut Op::CODE.encode());
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn push_value_chain<T: Encode>(mut self, val: &T) -> Self {
        let val = ScriptValue::new().set_value_chain(val);
        self.data.append(&mut val.to_script());
        self
    }

    // FIXME remove #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn push_op_code_chain<Op: OpCode>(mut self) -> Self {
        self.data.append(&mut Op::CODE.encode());
        self
    }

    pub fn try_next_opcode(&mut self) -> Result<Option<u32>, ScriptError> {
        if self.data.len() - self.index != 0 {
            Ok(Some(
                u32::decode(self).map_err(|_| ScriptError::CannotDecodeOpCode)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn try_next_value(&mut self) -> Result<Option<ScriptValue>, ScriptError> {
        if self.data.len() - self.index != 0 {
            Ok(Some(
                ScriptValue::decode(self).map_err(|_| ScriptError::CannotDecodeValue)?,
            ))
        } else {
            Ok(None)
        }
    }
}

impl Input for Script {
    fn remaining_len(&mut self) -> Result<Option<usize>, Error> {
        Ok(Some(self.data.len() - self.index))
    }

    fn read(&mut self, into: &mut [u8]) -> Result<(), Error> {
        if into.len() > self.data.len() - self.index {
            return Err("Not enough data to fill buffer".into());
        }
        let len = into.len();
        into.copy_from_slice(&self.data[self.index..self.index + len]);
        self.index += len;
        Ok(())
    }
}

pub mod tests {
    use super::*;

    pub fn default_script() -> Script {
        Script::new()
            .push_value_chain(&1_u64)
            .push_value_chain(&2_u64)
            .push_value_chain(&3_u64)
            .push_value_chain(&4_u64)
    }
}
