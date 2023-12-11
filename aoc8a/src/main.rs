// Import necessary modules and libraries
use num_integer::lcm;
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

    // Initialize variable to store current path len
    let mut path_len: usize = 0;

    // Initialize a hash map to store left mapping
    let mut left_map: HashMap<String, String> = HashMap::new();

    // Initialize a hash map to store right mapping
    let mut right_map: HashMap<String, String> = HashMap::new();

    // Initialize a counter variable
    let mut counter: usize = 0;

    // Initialize list of directions
    let mut directions: Vec<char> = Vec::new();

    // Initialize current positions
    let mut currents: Vec<String> = Vec::new();

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
            let modified_line: String = line
                .replace(" ", "")
                .replace("(", "")
                .replace(")", "")
                .replace(",", "=");
            let line_split: Vec<&str> = modified_line.split("=").collect();
            left_map.insert(line_split[0].to_string(), line_split[1].to_string());
            right_map.insert(line_split[0].to_string(), line_split[2].to_string());
            if line_split[0].ends_with("A") {
                currents.push(line_split[0].to_string());
            }
        }
        counter += 1;
    }

    // Initialize vector to store lengths of paths taken by all routes
    let mut path_len_vec: Vec<usize> = vec![0; currents.len()];

    // Loop indefinitely until ??Z is reached for all paths individually
    loop {
        let direction = directions[path_len % directions.len()];
        path_len += 1;

        // Look at direction and choose next location accordingly
        for i in 0..currents.len() {
            if path_len_vec[i] == 0 {
                let current = &currents[i];

                if direction == 'L' {
                    currents[i] = left_map
                        .get(current)
                        .expect("Retrieving failed")
                        .to_string();
                } else {
                    currents[i] = right_map
                        .get(current)
                        .expect("Retrieving failed")
                        .to_string();
                }

                // End loop for a path if it reaches ??Z
                if currents[i].ends_with("Z") {
                    path_len_vec[i] = path_len;
                }
            }
        }

        // End loop if all paths have a non-zero value
        let min_path_len_vec: usize = *path_len_vec.iter().min().expect("Minimizing failed");
        if min_path_len_vec != 0 as usize {
            break;
        }
    }

    // Initialize variable to store lcm of lengths of paths taken by all routes
    let mut lcm_path_len: usize = 1;

    // Calculate LCM for all paths
    for i in path_len_vec {
        lcm_path_len = lcm(lcm_path_len, i);
    }

    // Print final result
    println!("{:?}", lcm_path_len);
}
