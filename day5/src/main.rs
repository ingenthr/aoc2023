use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut almanac = String::new();

    f.read_to_string(&mut almanac)?;

    let seeds_line = almanac.lines().next();

    println!("{}", seeds_line.expect("Input should have at least one line."));

    // gather up the seeds to be planted in a Vec
    let seed_nums_agg = seeds_line.expect("Seed numbers should be after \"seeds:\", but there is no : char.")
        .split_terminator(":").nth(1);
    let seeds_nums = seed_nums_agg.expect("Expected whitespace between seed numbers.")
        .split_whitespace().collect::<Vec<&str>>();

    // build seed to soil map
    // TODO: extract this out to a function
    let seed_to_soil: HashMap<i32, i32> = HashMap::new();
    let s_to_s_in = almanac
        .split_terminator("seed-to-soil map:")
        .nth(1)
        .expect("Expected a seed-to-soil map line.")
        .split_terminator("\n\n")
        .nth(0)
        .expect("Expected to get multiple lines for seed-to-soil mapping")
        .split_terminator("\n");
    
    for line in s_to_s_in {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 3 {
            // skips past the extra line
            let drange_start = u32::from_str_radix(parts[0], 10).expect("Expected a destination range number.");
            let srange_start = u32::from_str_radix(parts[1], 10).expect("Expected a start range number.");
            let range_len = u32::from_str_radix(parts[2], 10).expect("Expected a range length number.");

            println!("Found a destination, start, length of {} {} {}", drange_start, srange_start, range_len);
        }
    }

    // println!("seed to soil strings {}", s_to_s_in);
    // dbg!(s_to_s_in);

    Ok(())

}

