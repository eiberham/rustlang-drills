pub struct Data {
    sender: String,
    receiver: String,
    amount: u32
}

pub struct Block {
    pub hash: String,
    pub previous_block_hash: String,
    pub data: Data
}

fn main() {
    println!("Hello, blockchain!");
}
