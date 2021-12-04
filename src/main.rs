use std::io;
use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
};

use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {

    let values = get_values("data/test.jsonl")?;

    let contents = serde_json::to_string(&values)?;
    let mut file = File::create("foo.txt")?;
    file.write(contents.as_bytes())?;

    Ok(())
}

pub fn get_values(filename: &str) -> Result<Vec<Value>> {
    let mut values = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(json) = line {
                let value: Value = serde_json::from_str(&json)?;

                values.push(value);
            }
        }
    }
    Ok(values)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
