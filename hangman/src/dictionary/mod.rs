extern crate rand;

pub use std::{fs, io};

pub mod words {
    use rand::{self, Rng};

    pub fn get() -> Result<String, String> {
        let data = super::file::read("src/dictionary/words.txt").unwrap();
        let mut rng = rand::thread_rng();
        let lines: Vec<&str> = data.lines().collect();
        let num: usize = rng.gen_range(0..lines.len());

        // ok_or transforms Option<T> into Result<T, E>
        // lines.get(num).ok_or("error")

        match lines.get(num) {
            None => return Err("ups, couldn't get a word".to_string()),
            Some(word) => Ok(
                String::from(*word)
            ),
        }
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
