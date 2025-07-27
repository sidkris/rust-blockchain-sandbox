use chrono::Utc;
use sha2::{Sha256, Digest};

const DIFFICULTY_PREFIX : &str = "00000";

fn main() {

    let transaction_1 = Transaction{
        sender : "Alice".to_string(),
        receiver : "Bob".to_string(),
        amount : 10.0,
    };

    let transaction_2 = Transaction{
        sender : "Bob".to_string(),
        receiver : "Charles".to_string(),
        amount : 5.0,
    };

    let mut blockchain= Blockchain::new();
    blockchain.add_block(&[transaction_1, transaction_2]);

    let latest_block = blockchain.chain.last().unwrap();

    println!("Block Hash : {}", latest_block.hash);
    println!("Block Height : {}", latest_block.height);
    println!("Block Nonce : {}", latest_block.nonce);

    println!("\nValidating the Block..");

    if blockchain.is_chain_valid() {
        println!("The Block is Valid.")
    }
    else {
        println!("This is an Invalid Block.");
    }

}

#[derive(Clone)]
pub struct Transaction {
    pub sender : String,
    pub receiver : String,
    pub amount : f64,
}

pub struct Block {
    pub timestamp : i64,                    // integer representation of the time when the block was created
    pub pre_block_hash : String,            // a string containing the hash value of the previous block in the blockchain
    pub hash : String,                      // a string containing the hash value of the current block
    pub transactions : Vec<Transaction>,    // a vector that holds the various transactions included in the block
    pub nonce : i64,                        // nonce : number used only once - a value miners change while mining a block to find a hash
    pub height : usize,                     // a value that indicates the position of the current block within the blockchain (represents the number of blocks that come before the current block)
}


pub fn create_new_block(pre_block_hash : String, transactions : &[Transaction], height : usize) -> Block {
    
    let timestamp = current_timestamp();
    
    let mut block = Block{
        timestamp,
        pre_block_hash : pre_block_hash.clone(),
        hash : String::new(),
        transactions : transactions.to_vec(),
        nonce : 0,
        height,
    };

    mine_block(&mut block);
    block
}


fn calculate_hash(block : &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.timestamp.to_string());
    hasher.update(&block.pre_block_hash);
    hasher.update(block.nonce.to_string());
    hasher.update(block.height.to_string());

    for i in &block.transactions {
        hasher.update(&i.sender);
        hasher.update(&i.receiver);
        hasher.update(&i.amount.to_string());
    }

    let output_hash = hasher.finalize();
    hex::encode(output_hash)

}

fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

fn mine_block(block : &mut Block) {
    println!("Mining Block..");
    loop {
        let hash = calculate_hash(block);
        if hash.starts_with(DIFFICULTY_PREFIX) {
            block.hash = hash;
            break;
        }
        else {
            block.nonce += 1;
        }
    }
    println!("Block Mined Successfully.");
}


pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {

pub fn new() -> Self {
    let genesis_block = create_new_block("0".to_string(), &[], 0);
    Blockchain { chain: vec![genesis_block] }
}
    

pub fn add_block(&mut self, transactions : &[Transaction]) {
    let previous = self.chain.last().unwrap();
    let new_block = create_new_block(previous.hash.clone(), transactions, self.chain.len());
    self.chain.push(new_block);
}

pub fn is_chain_valid(&self) -> bool {
    for i in 1..self.chain.len(){
        let current = &self.chain[i];
        let previous = &self.chain[i - 1];

        if current.pre_block_hash != previous.hash {
            return false;
        }

        if calculate_hash(current) != current.hash {
            return false;
        }
    }
    true
}

}