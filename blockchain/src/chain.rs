use crate::block::Block;

pub struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    pub fn new(genesis: Block) -> Self {
        Chain {
            chain: vec![genesis],
        }
    }

    pub fn generate_new_block(&mut self) -> &Block {
        let tip = self.tip();
        let block = Block::new(tip.number() + 1, tip.hash());
        self.chain.push(block);
        self.tip()
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

        let tip = chain.generate_new_block().clone();

        assert_eq!(tip.number(), genesis.number() + 1);
        assert_eq!(tip.prev_hash(), genesis.hash());
        assert_ne!(chain.tip(), chain.genesis());

        assert_eq!(chain.get_block(genesis.number()), Some(&genesis));
        assert_eq!(chain.get_block(tip.number()), Some(&tip));
        assert_eq!(chain.get_block(tip.number() + 1), None);
    }
}
