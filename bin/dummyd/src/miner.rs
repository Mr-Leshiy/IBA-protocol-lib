use crate::{block::Block, transaction::Transaction};
use std::{thread::sleep, time::Duration};

pub fn generate_block(prev_block: &Block, transaction: Vec<Transaction>) -> Block {
    sleep(Duration::from_secs(5));

    Block::new(prev_block.number() + 1, prev_block.hash(), transaction)
}
