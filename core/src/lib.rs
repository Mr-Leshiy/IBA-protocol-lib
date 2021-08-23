use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionData {
    size : u32,
    data : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    version : u32,
    timestamp : u64,
    executed_data : ActionData,
    condition_data : ActionData
}

impl PartialEq for ActionData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size &&
        self.data == other.data
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version &&
        self.timestamp == other.timestamp &&
        self.executed_data == other.executed_data &&
        self.condition_data == other.condition_data
    }
}

mod tests
{
    use crate::ActionData;
    use crate::Transaction;

    #[test]
    fn serialization_test() {
        let transaction = Transaction { version : 1, timestamp : 2, executed_data : ActionData { size: 1, data: vec![9] }, condition_data : ActionData { size: 1, data: vec![9] }};
        let serialized = bincode::serialize(&transaction).unwrap();
        let deserialized: Transaction = bincode::deserialize(&serialized).unwrap();

        assert_eq!(transaction, deserialized);
    }
}