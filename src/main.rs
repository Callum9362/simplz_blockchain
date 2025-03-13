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
        let genesis_block = Block::new(0, "Genesis Block".to_owned(), String::new());
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

    println!("{:#?}", blockchain);
}