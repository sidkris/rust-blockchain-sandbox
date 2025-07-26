fn main() {

}


pub struct Block {
    timestamp : i64,                    // integer representation of the time when the block was created
    pre_block_hash : String,            // a string containing the hash value of the previous block in the blockchain
    hash : String,                      // a string containing the hash value of the current block
    transactions : Vec<Transactions>,    // a vector that holds the various transactions included in the block
    nonce : i64,                        // nonce : number used only once - a value miners change while mining a block to find a hash
    height : usize,                     // a value that indicates the position of the current block within the blockchain (represents the number of blocks that come before the current block)
}

pub fn create_new_block(pre_block_hash : String, transactions : &[Transactions], height : usize) -> Block {
    let mut block = Block{
        timestamp:crate::current_timestamp(),
        pre_block_hash,
        hash::String::new(),
        transactions:transactions.to_vec(),
        nonce : 0,
        height
    };
}


