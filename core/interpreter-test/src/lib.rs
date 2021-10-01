#[cfg(test)]
mod tests {
    use interpreter::interpret;
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

        let script = default_script();
        interpret!(script, OpTest {}).unwrap();
    }

    #[test]
    fn op_add_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpAdd>();

        let res = interpret!(script).unwrap().unwrap().get_value::<u64>();
        assert_eq!(res, Ok(11));
    }

    #[test]
    fn op_sub_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpSub>();

        let res = interpret!(script).unwrap().unwrap().get_value::<u64>();
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn op_eql_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&6_u64);
        script.push_op_code::<OpEql>();

        let res = interpret!(script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));

        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpEql>();

        let res = interpret!(script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(false));
    }

    #[test]
    fn op_nql_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&6_u64);
        script.push_op_code::<OpEql>();

        let res = interpret!(script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));

        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&5_u64);
        script.push_op_code::<OpEql>();

        let res = interpret!(script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(false));
    }

    #[test]
    fn interpret_test() {
        let mut script = Script::new();
        script.push_value(&6_u64);
        script.push_value(&8_u64);
        script.push_op_code::<OpAdd>();
        script.push_value(&12_u64);
        script.push_op_code::<OpSub>();
        script.push_value(&2_u64);
        script.push_op_code::<OpEql>();

        let res = interpret!(script).unwrap().unwrap().get_value::<bool>();
        assert_eq!(res, Ok(true));
    }
}
