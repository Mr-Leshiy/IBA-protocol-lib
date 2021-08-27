use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    version: u32,
    timestamp: u64,
    executed_data: Vec<u8>,
    condition_data: Vec<u8>,
}
// TODO : implement custom serialize/deserialize with u32 as vector size instead of usize

mod tests {
    use crate::Transaction;

    #[test]
    fn ser_test() {
        let transaction = Transaction {
            version: 1,
            timestamp: 2,
            executed_data: vec![9, 9],
            condition_data: vec![9, 9],
        };
        let expected_serialized = [
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 9, 2, 0, 0, 0, 0, 0, 0,
            0, 9, 9,
        ];
        let serialized = bincode::serialize(&transaction).unwrap();
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
        let serialized = [
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 9, 2, 0, 0, 0, 0, 0, 0,
            0, 9, 9,
        ];
        let deserialized: Transaction = bincode::deserialize(&serialized[..]).unwrap();
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
        let ser = bincode::serialize(&transaction).unwrap();
        let de: Transaction = bincode::deserialize(&ser[..]).unwrap();
        assert_eq!(transaction, de);
    }
}
