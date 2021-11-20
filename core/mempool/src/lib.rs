#![allow(dead_code)]
#![allow(unused_variables)]

use transaction::Transaction;

pub struct Mempool {
    pool: Vec<Transaction>,
}

impl Mempool {
    fn new() -> Self {
        Mempool { pool: Vec::new() }
    }

    pub fn submit(&mut self, transaction: Transaction) {
        self.pool.push(transaction);
    }

    pub fn ready(&self) -> Option<Transaction> {
        for tr in self.pool.iter() {
            if Mempool::is_valid(tr) {
                return Some(tr.clone());
            }
        }
        None
    }

    pub fn remove_invalid(&mut self) {
        unimplemented!()
    }

    fn is_valid(transaction: &Transaction) -> bool {
        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use script::tests::default_script;

    pub fn default_transaction() -> Transaction {
        Transaction::new(1, 2, default_script(), default_script())
    }

    #[test]
    fn submit_and_get_transaction() {
        let mut mempool = Mempool::new();
        let tr = default_transaction();
        mempool.submit(tr);
        assert_eq!(mempool.ready().unwrap(), default_transaction());
    }
}
