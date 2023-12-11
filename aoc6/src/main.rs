// Import necessary modules and libraries
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?; // Try to open the file
    Ok(io::BufReader::new(file).lines()) // Return a buffered reader for the file lines
}

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Initialize a line counter
    let mut counter: u32 = 0;

    // Initialize final product of ways variable
    let mut ways_product: u32 = 1;

    // Initialize vectors for game times and records
    let (mut times, mut records): (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Parse first line
        if counter == 0 {
            times = line
                .split_whitespace()
                .skip(1)
                .map(|c| {
                    c.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Parsing failed: {}", line))
                })
                .collect();
            counter = 1;
            continue;
        }
        // Parse second line
        if counter == 1 {
            records = line
                .split_whitespace()
                .skip(1)
                .map(|c| {
                    c.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Parsing failed: {}", line))
                })
                .collect();
        }
    }

    // Calculate ways to beat the record
    for i in 0..times.len() {
        let time: u32 = times[i];
        let record: u32 = records[i];
        let mut n_ways: u32 = 0;

        // Iterate over possible completion times for the game
        for j in 1..time {
            let score: u32 = j * 0 + j * (time - j);
            if score > record {
                n_ways += 1;
            }
        }
        ways_product *= n_ways;
    }

    // Print the final result
    println!("{:?}", ways_product);
}
