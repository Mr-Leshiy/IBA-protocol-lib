use parity_scale_codec::{Decode, Encode};

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct Transaction {
    #[codec(compact)]
    version: u32,
    #[codec(compact)]
    timestamp: u64,
    executed_data: Vec<u8>,
    condition_data: Vec<u8>,
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
        let expected_serialized: &[u8] = &[4, 8, 8, 9, 9, 8, 9, 9];
        let serialized = transaction.encode();
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
        let mut serialized: &[u8] = &[4, 8, 8, 9, 9, 8, 9, 9];
        let deserialized = Transaction::decode(&mut serialized).unwrap();
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
        let mut serialized: &[u8] = &transaction.encode();
        let deserialized = Transaction::decode(&mut serialized).unwrap();
        assert_eq!(transaction, deserialized);
    }
}
