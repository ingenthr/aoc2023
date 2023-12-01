use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;


// TODO: return the value to main, print there
// TODO: read front and back in parallel?


fn main() -> std::io::Result<()> {
    sum_calvals()
}

fn sum_calvals() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

    let mut values: Vec<u64> = Vec::new();
    let mut res_string: String;

    for line in lines {
        res_string = String::new();
        let mut nums: Vec<char> = Vec::new();
        for char in line.chars() {
            // take the first and last number
            if char.is_numeric() {
                nums.push(char);
            }
        }
        res_string.push(nums[0]);
        res_string.push(nums[nums.len()-1]);
        values.push(res_string.parse::<u64>().unwrap());
    }

    let mut sum = 0;
    for val in values {
        sum = sum+val;
    }

    println!("{}", sum);

    Ok(())

}