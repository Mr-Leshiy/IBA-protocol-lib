use super::block::Block;
use super::chain::Chain;
use super::execution::execute_block;
use super::miner::generate_block;

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
            let new_block = generate_block(self.active_chain.tip(), Vec::new());
            println!("mining new block: {} \n", new_block);

            println!("executing block: {} \n", new_block);
            match execute_block(&new_block) {
                Ok(_) => {
                    self.active_chain.set_tip(new_block).unwrap();
                    println!("new tip: {} \n", self.active_chain.tip());
                }
                Err(err) => println!("block: {}, is invalid, err: {}", new_block, &err),
            }
        }
    }
}
