use crate::{
    block::Block,
    transaction::{Transaction, TransactionError},
};

#[derive(Debug)]
pub enum ExecutionError {
    InvalidBlock,
    InvalidTransaction(TransactionError),
}

pub fn execute_block(block: &Block) -> Result<(), ExecutionError> {
    block
        .transactions()
        .iter()
        .try_for_each(execute_transaction)
}

pub fn execute_transaction(tx: &Transaction) -> Result<(), ExecutionError> {
    tx.execute().map_err(ExecutionError::InvalidTransaction)
}
