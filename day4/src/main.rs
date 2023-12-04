use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;


    let mut points = 0;

    for line in lines {
        let row_nums = line.split_terminator(":").nth(1);
        let row_parts = row_nums.expect("Row had no colon.").split_terminator("|").collect::<Vec<&str>>();

        let winners = row_parts[0].split_whitespace().collect::<Vec<&str>>();
        let to_check = row_parts[1].split_whitespace().collect::<Vec<&str>>();

        let mut line_points = 0;
        for check in to_check {
            for winner in &winners {
                if &check == winner {
                    if line_points == 0 {
                        line_points = 1;
                    } else {
                        line_points = line_points * 2;
                    }
                }
            }
        }
        points = points + line_points;
    }

    println!("Answer: {}", points);

    Ok(())

}

