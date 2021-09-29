#[cfg(test)]
mod tests {
    use interpreter::interpret;
    use script::{opcode::OpCode, tests::default_script};

    #[test]
    fn interpret_compilation_test() {
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
        interpret!(script, OpTest {});
    }
}
