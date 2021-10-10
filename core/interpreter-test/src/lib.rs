#[cfg(test)]
mod tests {
    use interpreter::gen_interpreter;
    use script::{
        opcode::{OpAdd, OpCode, OpEql, OpSub},
        tests::default_script,
        Script,
    };

    #[test]
    fn interpret_codegen_test() {
        struct OpTest {}
        impl OpCode for OpTest {
            const CODE: u32 = 10;
            type Args = ();
            type Res = ();

            fn handler(args: Self::Args) -> Self::Res {
                args
            }
        }

        let mut script = default_script();
        let interpret = gen_interpreter!(OpTest {});
        interpret(&mut script).unwrap();
    }

    #[test]
    fn op_add_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpAdd>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<u64>();
        assert_eq!(res, Ok(11));
    }

    #[test]
    fn op_sub_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpSub>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<u64>();
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn op_eql_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&6_u64);
        script.push_op_code::<OpEql>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));

        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpEql>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(false));
    }

    #[test]
    fn op_nql_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&6_u64);
        script.push_op_code::<OpEql>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));

        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpEql>();

        let interpret = gen_interpreter!();
        let res = interpret(&mut script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(false));
    }

    #[test]
    fn interpret_test() {
        struct OpSquared {}
        impl OpCode for OpSquared {
            const CODE: u32 = 10;
            type Args = u64;
            type Res = u64;

            fn handler(arg: Self::Args) -> Self::Res {
                arg * arg
            }
        }

        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&8_u64);
        script.push_op_code::<OpAdd>();
        script.push_value(&12_u64);
        script.push_op_code::<OpSub>();
        script.push_op_code::<OpSquared>();
        script.push_value(&4_u64);
        script.push_op_code::<OpEql>();

        let interpret = gen_interpreter!(OpSquared {});
        let res = interpret(&mut script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));
    }
}
