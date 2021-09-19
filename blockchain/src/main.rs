mod blockchain;
use crate::blockchain::*;

fn main() {
    println!("Hello, blockchain!");

    let block = Block::new(
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
}
