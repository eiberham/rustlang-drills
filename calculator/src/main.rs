use std::io;

fn main() -> () {
    println!("Welcome to calculator!");
    println!("Enter a number to choose your desired operation.");
    println!("1. Addition.");
    println!("2. Substraction.");
    println!("3. Product.");
    println!("4. Division.");

    let mut input = String::new();

    let mut result: i32 = 0;

    io::stdin().read_line(&mut input).expect("Error message");

    let operation: i32 = input.trim().parse::<i32>().unwrap();
    input = String::new();

    println!("Please enter first operand: ");
    io::stdin().read_line(&mut input).expect("Error message");

    let a: i32 = input.trim().parse::<i32>().unwrap();
    input = String::new();

    println!("Please enter second operand: ");
    io::stdin().read_line(&mut input).expect("Error message");

    let b: i32 = input.trim().parse::<i32>().unwrap();

    match operation {
        1 => result = add(a, b),
        2 => result = substract(a, b),
        3 => result = multiply(a, b),
        4 => result = divide(a, b),
        _ => println!("Operation not found")
    }

    println!("the outcome is {}", result);

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn substract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    assert!(b > 0);

    a / b
}
