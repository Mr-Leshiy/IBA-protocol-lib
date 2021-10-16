use crate::{
    block::Block, chain::Chain, execution::execute_block, miner::generate_block,
    transaction::Transaction,
};

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

        println!("genesis: {}\n", self.active_chain.genesis());
        println!("-------------");

        loop {
            // TODO: take transactions from the mempool
            // TODO: provide a current timestamp
            let tx = Transaction::new(0);

            let new_block = generate_block(self.active_chain.tip(), vec![tx]);
            println!("mining new block: {} \n", new_block);

            println!("executing block: {} \n", new_block);
            match execute_block(&new_block) {
                Ok(_) => {
                    self.active_chain.set_tip(new_block).unwrap();
                    println!("new tip: {} \n", self.active_chain.tip());
                }
                Err(err) => println!("block: {}, is invalid, err: {:?}\n", new_block, &err),
            }
            println!("-------------");
        }
    }
}
