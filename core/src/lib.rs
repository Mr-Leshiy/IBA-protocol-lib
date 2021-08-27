use bincode::Options;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    version: u32,
    timestamp: u64,
    executed_data: Vec<u8>,
    condition_data: Vec<u8>,
}

// TODO : implement custom serialize/deserialize with u32 as vector size instead of usize
impl Transaction {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, bincode::Error> {
        bincode::deserialize(&bytes)
    }

    pub fn get_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }
}

mod tests {
    use super::*;

    #[test]
    fn ser_test() {
        let transaction = Transaction {
            version: 1,
            timestamp: 2,
            executed_data: vec![9, 9],
            condition_data: vec![9, 9],
        };
        let expected_serialized = vec![
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 9, 2, 0, 0, 0, 0, 0, 0,
            0, 9, 9,
        ];
        let serialized = transaction.get_bytes().unwrap();
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn de_test() {
        let expected_deserialized = Transaction {
            version: 1,
            timestamp: 2,
            executed_data: vec![9, 9],
            condition_data: vec![9, 9],
        };
        let serialized = vec![
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 9, 2, 0, 0, 0, 0, 0, 0,
            0, 9, 9,
        ];
        let deserialized = Transaction::from_bytes(serialized).unwrap();
        assert_eq!(deserialized, expected_deserialized);
    }

    #[test]
    fn ser_de_test() {
        let transaction = Transaction {
            version: 1,
            timestamp: 256,
            executed_data: vec![],
            condition_data: vec![8, 8, 8],
        };
        let ser = transaction.get_bytes().unwrap();
        let de = Transaction::from_bytes(ser).unwrap();
        assert_eq!(transaction, de);
    }
}
