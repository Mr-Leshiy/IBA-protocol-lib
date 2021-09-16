use iba_lib::core::transaction::Transaction as IbaTransaction;
use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::fmt::{Debug, Display};

#[derive(Encode, Decode, PartialEq, Clone, Debug)]
pub struct Transaction {
    lib_tx: IbaTransaction,
}

impl Transaction {
    pub fn hash(&self) -> [u8; 32] {
        self.lib_tx.hash()
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
