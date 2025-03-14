use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u32, data: String, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block(4);
        block
    }

    fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.prev_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        self.hash = self.calculate_hash();

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {

    fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_owned(),
            prev_hash: String::new(),
            hash: String::new(),
            nonce: 0,
        };
        let mut genesis_block = genesis_block;
        genesis_block.mine_block(4);
        Blockchain {
            chain: vec![genesis_block],
        }

    }

    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.latest_block().hash.clone();
        let new_block = Block::new(self.chain.len() as u32, data, prev_hash);
        self.chain.push(new_block);
    }

    fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Mining block 1...");
    blockchain.add_block("First block data".to_owned());

    println!("Mining block 2...");
    blockchain.add_block("Second block data".to_owned());

    if blockchain.is_valid_chain() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is INVALID!");
    }

    println!("{:#?}", blockchain);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(1, "Test Data".to_owned(), "PreviousHash".to_owned());

        assert_eq!(block.index, 1);
        assert_eq!(block.data, "Test Data");
        assert_eq!(block.prev_hash, "PreviousHash");
        assert!(block.hash.starts_with("0000"));
    }

    #[test]
    fn test_blockchain_initialization() {
        let blockchain = Blockchain::new();
        let genesis_block = &blockchain.chain[0];

        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.data, "Genesis Block");
        assert_eq!(genesis_block.prev_hash, "");
        assert!(genesis_block.hash.starts_with("0000"));
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("First block data".to_owned());

        assert_eq!(blockchain.chain.len(), 2);

        let latest_block = &blockchain.chain[1];
        let previous_block = &blockchain.chain[0];

        assert_eq!(latest_block.index, 1);
        assert_eq!(latest_block.data, "First block data");
        assert_eq!(latest_block.prev_hash, previous_block.hash);
        assert!(latest_block.hash.starts_with("0000"));
    }

    #[test]
    fn test_multiple_blocks() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("Block 1 data".to_owned());
        blockchain.add_block("Block 2 data".to_owned());
        blockchain.add_block("Block 3 data".to_owned());

        assert_eq!(blockchain.chain.len(), 4);
        assert!(blockchain.is_valid_chain());
    }

    #[test]
    fn test_genesis_block_consistency() {
        let blockchain_1 = Blockchain::new();
        let blockchain_2 = Blockchain::new();

        assert_eq!(blockchain_1.chain[0].hash, blockchain_2.chain[0].hash);
        assert_eq!(blockchain_1.chain[0].data, "Genesis Block");
        assert_eq!(blockchain_1.chain[0].prev_hash, "");
    }

    #[test]
    fn test_is_valid_chain() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("First block data".to_owned());
        blockchain.add_block("Second block data".to_owned());

        assert!(blockchain.is_valid_chain());
    }

    #[test]
    fn test_tampered_block_validation() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("First block data".to_owned());

        blockchain.chain[1].data = "Tampered Data".to_owned();

        assert!(!blockchain.is_valid_chain());
    }

    #[test]
    fn test_large_blockchain_performance() {
        let mut blockchain = Blockchain::new();

        for i in 1..=10 {
            blockchain.add_block(format!("Block {} data", i));
        }

        assert_eq!(blockchain.chain.len(), 11);
        assert!(blockchain.is_valid_chain());
    }
}
