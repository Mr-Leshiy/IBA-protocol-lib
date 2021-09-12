use crate::block::Block;

pub struct Chain {
    chain: Vec<Block>,
}

#[derive(Debug)]
pub enum ChainError {
    BlockCannotConnect(String),
}

impl Chain {
    pub fn new(genesis: Block) -> Self {
        Chain {
            chain: vec![genesis],
        }
    }

    pub fn set_tip(&mut self, tip: Block) -> Result<(), ChainError> {
        if tip.prev_hash() != self.tip().hash() {
            return Err(ChainError::BlockCannotConnect(format!(
                "provided block does not reference for the current tip. provided prev hash: {}, currnet tip hash: {}",
                hex::encode(tip.prev_hash()),
                hex::encode(self.tip().hash())
            )));
        }
        if tip.number() != self.tip().number() + 1 {
            return Err(ChainError::BlockCannotConnect(format!(
                "provided block does not continue for the current tip. provided block number: {}, expected number: {}",
                tip.number(),
                (self.tip().number() + 1)
            )));
        }

        self.chain.push(tip);
        Ok(())
    }

    pub fn tip(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn genesis(&self) -> &Block {
        self.chain.first().unwrap()
    }

    pub fn get_block(&self, number: u64) -> Option<&Block> {
        let genesis = self.genesis();
        self.chain.get((number - genesis.number()) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::miner::generate_block;

    fn genesis_block() -> Block {
        Block::new(0, [0; 32])
    }

    #[test]
    fn basic_chain_test() {
        let genesis = genesis_block();

        let mut chain = Chain::new(genesis.clone());

        assert_eq!(chain.tip(), chain.genesis());
        assert_eq!(chain.genesis(), &genesis);

        assert_eq!(chain.get_block(genesis.number()), Some(&genesis));
        assert_eq!(chain.get_block(genesis.number() + 1), None);

        let new_block = generate_block(chain.tip());
        chain.set_tip(new_block.clone());

        assert_eq!(new_block.number(), genesis.number() + 1);
        assert_eq!(new_block.prev_hash(), genesis.hash());
        assert_ne!(chain.tip(), chain.genesis());

        assert_eq!(chain.get_block(genesis.number()), Some(&genesis));
        assert_eq!(chain.get_block(new_block.number()), Some(&new_block));
        assert_eq!(chain.get_block(new_block.number() + 1), None);
    }
}
