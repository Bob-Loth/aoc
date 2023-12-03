pub mod input {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Result},
    };
    pub fn lines(i: i32) -> Result<Vec<String>> {
        BufReader::new(File::open(format!("src/bin/{i}/input.txt"))?)
            .lines()
            .collect()
    }
}
