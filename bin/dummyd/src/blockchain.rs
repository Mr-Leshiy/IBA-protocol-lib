use crate::block::Block;
use crate::chain::Chain;
use crate::miner::generate_block;

pub struct Blockchain {
    active_chain: Chain,
}

impl Blockchain {
    pub fn new(genesis: Block) -> Self {
        Self {
            active_chain: Chain::new(genesis),
        }
    }

    pub fn run(&mut self) {
        println!("Running blockchain");

        println!("genesis: {}", self.active_chain.genesis());

        loop {
            let new_block = generate_block(self.active_chain.tip());
            println!("mining new block: {} \n", new_block);

            self.active_chain.set_tip(new_block).unwrap();
            println!("new tip: {} \n", self.active_chain.tip());
        }
    }
}
