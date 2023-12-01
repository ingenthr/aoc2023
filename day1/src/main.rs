use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;



fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    read_in()
}

fn read_in() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

    for line in lines {
        println!("{}", line);
    }

    Ok(())

}