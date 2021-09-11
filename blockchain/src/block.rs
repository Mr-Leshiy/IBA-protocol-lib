use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Encode, Decode, PartialEq, Clone, Debug)]
pub struct BlockHeader {
    pub number: u64,
    pub prev_hash: [u8; 32],
    pub root_hash: [u8; 32],
}

impl BlockHeader {
    pub fn hash(&self) -> [u8; 32] {
        Sha256::new()
            .chain(self.encode())
            .finalize()
            .try_into()
            .map_err(|_| "Expected length of the array is 32")
            .unwrap()
    }
}

impl Display for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hash: {}, number: {}, prev_hash: {}, root_hash: {}",
            hex::encode(self.hash()),
            self.number,
            hex::encode(self.prev_hash),
            hex::encode(self.root_hash)
        )
    }
}

impl Block {
    pub fn new(number: u64, prev_hash: [u8; 32]) -> Block {
        let root_hash = [0; 32];
        Block {
            header: BlockHeader {
                number,
                prev_hash,
                root_hash,
            },
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "header: ({})", self.header)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    header: BlockHeader,
}

impl Block {
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    pub fn number(&self) -> u64 {
        self.header.number
    }

    pub fn prev_hash(&self) -> [u8; 32] {
        self.header.prev_hash
    }

    pub fn root_hash(&self) -> [u8; 32] {
        self.header.root_hash
    }

    pub fn hash(&self) -> [u8; 32] {
        self.header.hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_block_header() -> BlockHeader {
        BlockHeader {
            number: 0,
            prev_hash: [1; 32],
            root_hash: [2; 32],
        }
    }

    #[test]
    fn block_header_ser_test() {
        let block = default_block_header();
        let serialized = block.encode();
        let expected_serialized = hex::decode("000000000000000001010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202").unwrap();
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn block_header_de_test() {
        let block = default_block_header();
        let serialized = hex::decode("000000000000000001010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202").unwrap();
        let deserialized = BlockHeader::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(block, deserialized);
    }

    #[test]
    fn block_header_serde_test() {
        let block = default_block_header();
        let serialized = block.encode();
        let deserialized = BlockHeader::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(block, deserialized);
    }

    #[test]
    fn block_header_hash_test() {
        let block = default_block_header();
        let expected_hash: [u8; 32] =
            hex::decode("0aa07d23daebf0b061e7f4dd96908797c7e2078cc8d0abd172b9b33a873114b2")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(block.hash(), expected_hash);
    }
}
