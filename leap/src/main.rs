// given a year report if it's a leap year

use std::io;
use regex::Regex;

fn main() -> () {
    println!("Enter a year:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error message");

    if validate_year(&input) {

    } else {
        println!("Year format not valid.");
    }
}

fn validate_year(year: &String) -> bool {
    let re = Regex::new(r"\d{4}").unwrap();
    re.is_match(*year)
}

fn is_leap(year: &String) {

}
