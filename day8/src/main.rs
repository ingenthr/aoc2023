use std::fs;
use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let map_input = fs::read_to_string("input.txt").expect("Expected to be able to open and read `input.txt`.");
    let mut input_lines = map_input.lines();
    let path = input_lines.next();
    let location_lines = map_input.split_terminator("\n\n").nth(1).expect("Expected the set of locations");

    let mut locations = HashMap::new();

    for location in location_lines.lines() {
        let loc = location.split_terminator("=").nth(0).expect("Expected lines split by `=`").get(..3).expect("Expected 3 characters.");
        let loc_ptrs = location.split_terminator("=").nth(1).expect("Expected lines split by `=`"); // TODO:  toss above into a vec

        locations.insert(loc, loc_ptrs);
    }


    println!("Number of entries is {}.", locations.len());

    // walk the path starting with AAA and see how long it takes to get to ZZZ

    let mut fork = locations.get("AAA").expect("Expected a fork in the road (a set of paths)."); // starting point
    let mut iterations = 0;
    while fork != "ZZZ" {
        for direction in path.expect("Expected directions line to have characters.").chars() {
            // WIP
            // dbg!(fork);
            let fork_options = fork.get(2..10).expect("Expected more chars.").split(", ").collect::<Vec<&str>>();
            // dbg!(fork_options);

            if direction == 'L' {
                fork = locations.get(fork_options[0]).expect("Expected a string path.");
            } else if direction == 'R' {
                fork = locations.get(fork_options[1]).expect("Expected a string path.");
            } else {
                panic!("Expected either L or R.")
            }

            iterations += 1;
        }
    }

    println!("Took {} iterations to get to ZZZ", iterations);

    Ok(())

}
