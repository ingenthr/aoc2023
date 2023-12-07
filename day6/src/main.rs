use std::io;
use std::fs;

fn main() -> Result<(), io::Error> {
    let rrecords = fs::read_to_string("input.txt")?;

    let time_line = rrecords.lines().next().expect("Should have a line of times.");
    let dist_line = rrecords.lines().next().expect("Should have a line of distances.");

    let times = time_line.split_terminator(":")
        .nth(1)
        .expect("Expected times after a `:`.")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let dists = dist_line.split_terminator(":")
        .nth(1)
        .expect("Expected distances after a `:`.")
        .split_whitespace()
        .collect::<Vec<&str>>();
    println!("{}", time_line);

    for (i, dist) in dists.iter().enumerate() {
        println!("Column {} is {} using time {}", i, dist, times[i]); 
    }

    Ok(())

}
