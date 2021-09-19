use crate::core::script::Script;
use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
pub struct Transaction {
    #[codec(compact)]
    version: u32,
    #[codec(compact)]
    timestamp: u64,
    executed_script: Script,
    condition_script: Script,
}

impl Transaction {
    /// Calculates a Sha256 hash of the function
    pub fn hash(&self) -> [u8; 32] {
        Sha256::new()
            .chain(self.encode())
            .finalize()
            .try_into()
            .map_err(|_| "Expected length of the array is 32")
            .unwrap()
    }

    pub fn executed_script(&self) -> &Script {
        &self.executed_script
    }

    pub fn condition_script(&self) -> &Script {
        &self.condition_script
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::core::script::tests::default_script;

    fn default_transaction() -> Transaction {
        Transaction {
            version: 1,
            timestamp: 2,
            executed_script: default_script(),
            condition_script: default_script(),
        }
    }

    #[test]
    fn ser_test() {
        let transaction = default_transaction();
        let expected_serialized = hex::decode("0408d000000000200100000000000000000000002002000000000000000000000020030000000000000000000000200400000000000000d000000000200100000000000000000000002002000000000000000000000020030000000000000000000000200400000000000000").unwrap();
        let serialized = transaction.encode();
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn de_test() {
        let expected_deserialized = default_transaction();
        let serialized = hex::decode("0408d000000000200100000000000000000000002002000000000000000000000020030000000000000000000000200400000000000000d000000000200100000000000000000000002002000000000000000000000020030000000000000000000000200400000000000000").unwrap();
        let deserialized = Transaction::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(deserialized, expected_deserialized);
    }

    #[test]
    fn ser_de_test() {
        let transaction = default_transaction();
        let serialized = transaction.encode();
        let deserialized = Transaction::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(transaction, deserialized);
    }

    #[test]
    fn hash_test() {
        let transaction = default_transaction();
        let expected_hash: [u8; 32] =
            hex::decode("bac775dba80783ee668da044d0c4192f3094a50b664963f8b67b30cf2d742b2e")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(transaction.hash(), expected_hash);
    }
}
