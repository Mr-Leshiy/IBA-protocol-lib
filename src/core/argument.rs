use super::opcode::OP_PUSH;
use parity_scale_codec::{Decode, Encode, Error};

#[derive(Decode, Encode, PartialEq, Debug)]
pub struct Argument {
    data: Vec<u8>,
}

impl Argument {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    
    pub fn to_script(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut OP_PUSH.encode());
        data.append(&mut self.encode());
        data
    }

    pub fn set_value<T: Encode>(&mut self, val: T) {
        self.data = val.encode();
    }

    pub fn set_value_chain<T: Encode>(mut self, val: T) -> Self {
        self.data = val.encode();
        self
    }

    pub fn get_value<T: Decode>(&self) -> Result<T, Error> {
        T::decode(&mut self.data.as_ref())
    }
}
