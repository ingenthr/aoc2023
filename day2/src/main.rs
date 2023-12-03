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
    let mut maybe_game = String::new();

    let _ = lines.iter().map(|line| {
        // pluck out the possible sum
        let game_delim = line.find(":");
        maybe_game = (&line[4..game_delim.expect("Each line should have a single colon.")]).to_string();
        println!("{}", maybe_game)
    });


    // let mut sum = 0;
    // for val in values {
    //     sum = sum+val;
    // }

    // println!("{}", sum);

    Ok(())

}