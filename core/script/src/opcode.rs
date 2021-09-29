use crate::argument::Argument;
use impl_trait_for_tuples::impl_for_tuples;
use parity_scale_codec::{Decode, Encode};

#[derive(Debug)]
pub enum OpCodeError {
    InvalidArgumentAmount,
    UnexepectedArgumentType,
}

pub trait OpCode {
    type Args: OpCodeVal;
    type Res: OpCodeVal;
    const CODE: u32;

    fn handler(args: Self::Args) -> Self::Res;
}

pub trait OpCodeVal: Sized + Encode + Decode {
    fn decode_arguments(args_stack: &mut Vec<Argument>) -> Result<Self, OpCodeError> {
        args_stack
            .pop()
            .ok_or(OpCodeError::InvalidArgumentAmount)?
            .get_value()
            .map_err(|_| OpCodeError::UnexepectedArgumentType)
    }

    fn encode_arguments(self, args_stack: &mut Vec<Argument>) {
        args_stack.push(Argument::new().set_value_chain(self));
    }
}

impl OpCodeVal for u8 {}
impl OpCodeVal for u16 {}
impl OpCodeVal for u32 {}
impl OpCodeVal for u64 {}
impl OpCodeVal for u128 {}

impl OpCodeVal for i8 {}
impl OpCodeVal for i16 {}
impl OpCodeVal for i32 {}
impl OpCodeVal for i64 {}
impl OpCodeVal for i128 {}

impl OpCodeVal for bool {}

impl OpCodeVal for Argument {
    fn decode_arguments(args_stack: &mut Vec<Argument>) -> Result<Self, OpCodeError> {
        args_stack.pop().ok_or(OpCodeError::InvalidArgumentAmount)
    }

    fn encode_arguments(self, args_stack: &mut Vec<Argument>) {
        args_stack.push(self);
    }
}

#[impl_for_tuples(15)]
impl OpCodeVal for Tuple {
    fn decode_arguments(args_stack: &mut Vec<Argument>) -> Result<Self, OpCodeError> {
        let res = for_tuples!( ( #( Tuple::decode_arguments(args_stack)? ), *) );
        Ok(res)
    }

    fn encode_arguments(self, args_stack: &mut Vec<Argument>) {
        for_tuples!( #( Tuple.encode_arguments(args_stack); )* );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_arguments_test() {
        let mut args = Vec::new();

        args.push(Argument::new().set_value_chain(5_u64));
        args.push(Argument::new().set_value_chain(11_u64));

        let (val1, val2) = <(u64, u64)>::decode_arguments(&mut args).unwrap();
        assert_eq!(val1, 11_u64);
        assert_eq!(val2, 5_u64);
    }
}

pub struct OpPush;
pub struct OpEql;
pub struct OpNql;
pub struct OpAdd;
pub struct OpSub;

impl OpCode for OpPush {
    type Args = Argument;
    type Res = Argument;
    const CODE: u32 = 0;

    fn handler(args: Self::Args) -> Self::Res {
        args
    }
}

impl OpCode for OpEql {
    type Args = (Argument, Argument);
    type Res = bool;
    const CODE: u32 = 1;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 == args.1
    }
}

impl OpCode for OpNql {
    type Args = (Argument, Argument);
    type Res = bool;
    const CODE: u32 = 2;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 != args.1
    }
}

impl OpCode for OpAdd {
    type Args = (u64, u64);
    type Res = u64;
    const CODE: u32 = 3;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 + args.1
    }
}

impl OpCode for OpSub {
    type Args = (u64, u64);
    type Res = u64;
    const CODE: u32 = 4;

    fn handler(args: Self::Args) -> Self::Res {
        args.1 - args.0
    }
}
