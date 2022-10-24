mod blockchain;
use crate::blockchain::*;

fn main() {
    println!("Hello, blockchain!");

    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        Payload {
            amount: 100,
            receiver: String::from("Jenny Doe"),
            sender: String::from("John Doe"),
        },
        vec![0; 32],
        0
    );

    println!("{:?}", block);

    block.calculate_hash("Hello bitcoin");

    println!("{:?}", block);
}
