#[cfg(test)]
mod tests {
    use interpreter::eval;
    use script::{opcode::OpAdd, tests::default_script};

    #[test]
    fn eval_test() {
        let script = default_script();
        eval!(script, OpAdd {});
    }
}
