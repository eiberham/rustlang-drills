// To get in touch with how structs work in rust, make a program that asks 
// user to input information about his animal (dog), keep this information 
// stored in a struct type and add methods to mimic a dog's behaviour.

use std::io;

#[derive(Debug)]
struct Dog {
    name: String,
    age: u32,
    alive: bool,
}

fn main() {
    println!("Welcome! Please answer the following questions about ur dog.");
    println!("Enter your dog's name: ");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Couldn't read input!");

    println!("Enter your dog's age: ");

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Couldn't read input!");

    let lifetime = match age.trim().parse::<u32>() {
        Ok(lifetime) => lifetime,
        Err(e) => {
            println!("Please input a valid age ({})", e);
            return;
        }
    };

    let doggy = Dog {
        name,
        age: lifetime,
        alive: true
    };

    println!("{:?}", doggy);
}
