pub use std::{fs, io};
pub use rand::seq::IteratorRandom;

// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

pub mod words {
    pub fn get() -> () {
        let data = super::file::read("src/dictionary/words.txt").unwrap();
        let lines: Vec<&str> = data.lines().collect();
        lines.iter().map(|l| l.expect("Couldn't read line")).choose(&mut rand::thread_rng())
        .expect("File had no lines")
        /* for line in lines {
            println!("{:?}", line);
        } */
        // println!("{:?}", data)
    }
}

pub mod file {
    pub fn read(filepath: &str) -> Result<String, std::io::Error> {
         
        match std::fs::read_to_string(filepath) {
            Ok(content) => Ok(content),
            Err(err) => Err(err)
        }

        /*
         * these are fancy shortcuts analogous to the code above.
         * unwrap is a helper method of the Result<T, E> type that returns the value inside 
         * the Ok() or panics on Err(). 
         * let content = std::fs::read_to_string(filepath).unwrap(); 
         * or if you want to set a custom panic! message you can use the expect helper ...
         * let content = std::fs::read_to_string(filepath)
         * .expect("something bad happened.");
         * we can also return the error using the ? operator like this
         * std::fs::read_to_string(filepath)?
         * this operator can only be used in functions that return Result.
         */ 

    }
}
