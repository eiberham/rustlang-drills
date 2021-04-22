pub use std::fs;
pub use std::result;

pub mod words {
    pub fn get() {
        let data = super::file::read("src/dictionary/words.txt");
        println!("{:?}", data)
    }
}

pub mod file {
    pub fn read(filepath: &str) -> Result<String> {
         
        let content = match std::fs::read_to_string(filepath) {
            Ok(content) => content,
            Err(err) => return Err(err)
        };
    }
}
