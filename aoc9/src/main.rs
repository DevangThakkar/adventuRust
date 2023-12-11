// Import necessary modules and libraries
use itertools::Itertools;
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

    // Initialize variable to hold the sum of added values
    let mut sum: isize = 0;

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Split line over whitespace and map each individual item to isize
        let line_split: Vec<isize> = line
            .split(" ")
            .map(|c| {
                c.parse::<isize>()
                    .unwrap_or_else(|_| panic!("Parsing failed: {}", line))
            })
            .collect();

        // Initialize current inputs and push vector from original line
        let mut inputs: Vec<Vec<isize>> = Vec::new();
        inputs.push(line_split.clone());

        // Define current input as the vector from the original line
        let mut current_input: Vec<isize> = line_split.clone();

        // Loop till the current vector is all zeros
        loop {
            // Initialize empty vector for the next line
            let mut next_input: Vec<isize> = Vec::new();

            // Generate next line as differences
            for i in 0..current_input.len() - 1 {
                next_input.push(current_input[i + 1] - current_input[i]);
            }

            // Push next input to vector of vectors and set next input as current input
            inputs.push(next_input.clone());
            current_input = next_input.clone();

            // Obtain number of unique elements in the current input
            // Error: copied() is used to convert the iterator of references to an iterator of values (to avoid the error)
            let unique_positions: Vec<isize> = current_input.iter().unique().copied().collect();

            // End loop if all values in current vector are 0
            if unique_positions.len() == 1 && current_input[0] == 0 {
                break;
            }
        }

        // Generate new element at the end of each vector
        for i in (0..inputs.len() - 1).rev() {
            let this_line = inputs[i].clone();
            let next_line = inputs[i + 1].clone();
            inputs[i].push(this_line[this_line.len() - 1] + next_line[next_line.len() - 1]);
        }
        sum += inputs[0][inputs[0].len() - 1];
    }

    // Print final result
    println!("{:?}", sum);
}
