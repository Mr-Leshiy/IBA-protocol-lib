pub mod block;
pub mod blockchain;
pub mod chain;
pub mod miner;

use block::Block;
use blockchain::Blockchain;

fn main() {
    let genesis = Block::new(0, [0; 32], Vec::new());

    let mut blockchain = Blockchain::new(genesis);
    blockchain.run();
}
