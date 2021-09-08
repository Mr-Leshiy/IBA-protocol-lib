use parity_scale_codec::{Decode, Encode};

#[derive(Decode, Encode, PartialEq, Debug)]
pub struct OpCode {
    code: u32,
}

pub static OP_PUSH: OpCode = OpCode { code: 0 };
pub static OP_ADD: OpCode = OpCode { code: 1 };
pub static OP_SUB: OpCode = OpCode { code: 2 };
pub static OP_EQL: OpCode = OpCode { code: 3 };
pub static OP_NQL: OpCode = OpCode { code: 4 };
