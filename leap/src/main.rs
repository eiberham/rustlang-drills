// given a year report if it's a leap year

use std::io;
use regex::Regex;

fn main() -> () {
    println!("Enter a year:");

    let mut input = String::new();

    // Reminder.
    // stdin reads the global buffer that handles the input stream and 
    // also implements the BufRead trait which has the read_line method. 
    // This takes a mutable String as an input buffer and reads all bytes from 
    // the stream until a newline byte is reached and appends them to the buffer. 
    // The #expect() method unwraps the Result; if it is an Err it will panic 
    // with the message and the cause.
    io::stdin().read_line(&mut input).expect("Couldn't read input!");

    if validate_year(&input) {
        let year_input = input.trim().parse::<u64>();

        let year = match year_input {
            Ok(year) => year,
            Err(e) => {
                println!("Please input a number ({})", e);
                return;
            }
        };

        if is_leap(year) {
            println!("Year {} is leap", year);
        } else {
            println!("Year {} is not leap", year);
        }
    } else {
        println!("Year format not valid.");
    }
}

fn validate_year(year: &String) -> bool {
    let re = Regex::new(r"\d{4}").unwrap();
    re.is_match(&*year)
}

fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
