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


    let block = create_new_block("000000".to_string(), &[transaction_1, transaction_2], 1);

    println!("Block Hash : {}", block.hash);
    println!("Block Height : {}", block.height);
    println!("Block Nonce : {}", block.nonce);

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