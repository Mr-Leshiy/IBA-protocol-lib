use crate::block::Block;
use crate::transaction::{Transaction, TransactionError};

pub enum ExecutionError {
    InvalidBlock,
    InvalidTransaction(TransactionError),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidBlock => write!(f, "Block is invalid"),
            Self::InvalidTransaction(_) => write!(f, "Transaction is invalid"),
        }
    }
}

pub fn execute_block(block: &Block) -> Result<(), ExecutionError> {
    block
        .transactions()
        .iter()
        .try_for_each(execute_transaction)
}

pub fn execute_transaction(tx: &Transaction) -> Result<(), ExecutionError> {
    tx.execute()
        .map_err(|e| ExecutionError::InvalidTransaction(e))
}
