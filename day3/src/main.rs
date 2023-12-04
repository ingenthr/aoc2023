use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;


fn main() -> std::io::Result<()> {
    sum_calvals()
}

fn sum_calvals() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;


    let schematic = Vec<Vec<char>>;

    let mut linec = 0;
    let mut charc = 0;
    for line in lines {
        for char in line.chars() {
            schematic[linec][charc] = char;

            // take the first and last number
            if char.is_numeric() {
            }
            charc +=1;
        }
        linec += 1;
    }

    Ok(())

}