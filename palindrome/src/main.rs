use std::io;

fn main() {
    println!("Check if a word is palindrome");
    println!("Please enter a word: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error message");
    let valid: bool = is_palindrome(input);

    if valid {
        println!("{} is palindrome !");
    } else {
        println!("{} is not palindrome!");
    }
}

fn is_palindrome(word: &String) -> bool {

}
