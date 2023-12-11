// Import necessary modules and libraries
use std::collections::HashMap;
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

    // Initialize variable to store length of path
    let mut path_len: usize = 0;

    // Initialize a hash map to store left mapping
    let mut left_map: HashMap<String, String> = HashMap::new();

    // Initialize a hash map to store right mapping
    let mut right_map: HashMap<String, String> = HashMap::new();

    // Initialize a counter variable
    let mut counter: usize = 0;

    // Initialize list of directions
    let mut directions: Vec<char> = Vec::new();

    // Initialize current position
    let mut current: String = "AAA".to_string();

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Get directions as a char vector from the first line
        if counter == 0 {
            directions = line.chars().collect();
            counter += 1;
            continue;
        }

        // Starting from the third line, get left/right values and store into hash maps
        if counter > 1 {
            let modified_line: String = line.replace(" ", "").replace("(", "").replace(")", "");
            let line_split: Vec<&str> = modified_line
                .split(|c: char| c == '=' || c == ',')
                .collect();
            left_map.insert(line_split[0].to_string(), line_split[1].to_string());
            right_map.insert(line_split[0].to_string(), line_split[2].to_string());
        }
        counter += 1;
    }

    // Loop indefinitely until ZZZ is reached
    loop {
        let direction = directions[path_len % directions.len()];
        path_len += 1;

        // Look at direction and choose next location accordingly
        if direction == 'L' {
            current = left_map
                .get(&current)
                .expect("Retrieving failed")
                .to_string();
        } else {
            current = right_map
                .get(&current)
                .expect("Retrieving failed")
                .to_string();
        }

        // Stop if ZZZ is reached
        if current == "ZZZ" {
            break;
        }
    }

    // Print final result
    println!("{:?}", path_len);
}
