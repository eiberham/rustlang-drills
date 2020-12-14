use std::io;

fn main() -> () {
    println!("Check if a word is palindrome.");
    println!("Please enter a word: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error message");

    // we pass in a reference of input string to avoid ownership issues.
    let valid: bool = is_palindrome(&input);

    if valid {
        println!("{} is palindrome !", input);
    } else {
        println!("{} is not palindrome !", input);
    }
}

// word is a reference to a string so its value won't get dropped when out 
// of scope.
fn is_palindrome(word: &String) -> bool {
    // we need to trim the input string because of the way read_line formats it.
    let reversed = word.trim().chars().rev().collect::<String>();
    *word.trim() == reversed
}
