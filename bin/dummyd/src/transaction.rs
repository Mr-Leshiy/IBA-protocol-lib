use interpreter::gen_interpreter;
use parity_scale_codec::{Decode, Encode};
use script::{opcode::OpCode, Script, ScriptError};
use sha2::{Digest, Sha256};
use std::{
    convert::TryInto,
    fmt::{Debug, Display},
};
use transaction::Transaction as IbaTransaction;

pub struct TransactionError {
    tx_hash: [u8; 32],
    err_type: TransactionErrorType,
}

impl Debug for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx hash: {}, err: {:?}",
            hex::encode(self.tx_hash),
            self.err_type
        )
    }
}

#[derive(Debug)]
pub enum TransactionErrorType {
    InvalidScript(ScriptError),
    InvalidEvaluation,
}

#[derive(Encode, Decode, PartialEq, Clone, Debug)]
pub struct Transaction {
    iba_tx: IbaTransaction,
}

struct OpEcho;

impl OpCode for OpEcho {
    type Args = ();
    type Res = ();
    const CODE: u32 = 5;

    fn handler(_args: Self::Args) -> Self::Res {
        println!("OpEcho !!! \n");
    }
}

impl Transaction {
    pub fn new(timestamp: u64) -> Self {
        let version = 0;
        let executed_script = Script::new().push_op_code_chain::<OpEcho>();
        let conditional_script = Script::new();

        Self {
            iba_tx: IbaTransaction::new(version, timestamp, executed_script, conditional_script),
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        self.iba_tx.hash()
    }

    pub fn execute(&self) -> Result<(), TransactionError> {
        let mut script = self.iba_tx.executed_script().clone();

        let interpret = gen_interpreter!(OpEcho {});

        match interpret(&mut script).map_err(|err| TransactionError {
            tx_hash: self.hash(),
            err_type: TransactionErrorType::InvalidScript(err),
        })? {
            Some(_) => Err(TransactionError {
                tx_hash: self.hash(),
                err_type: TransactionErrorType::InvalidEvaluation,
            }),
            None => Ok(()),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transaction hash: ({})", hex::encode(self.hash()))
    }
}

// calculate a sha256 hash from the transaction hashes
pub fn calculate_root_hash(transactions: &[Transaction]) -> [u8; 32] {
    let mut data = Vec::new();
    transactions.iter().for_each(|tx| {
        data.append(&mut tx.hash().to_vec());
    });

    Sha256::new()
        .chain(data)
        .finalize()
        .try_into()
        .map_err(|_| "Expected length of the array is 32")
        .unwrap()
}
