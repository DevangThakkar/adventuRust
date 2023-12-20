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

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Initialize vector of Strings to store the system in the input
    let mut system: Vec<String> = Vec::new();

    // Initialize variable to store final sum
    let mut sum: usize = 0;

    // Iterate over each line in the file to read notes
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");
        system.push(line);
    }

    // Transpose matrix so we can look at it row by row instead of column by column
    let mut transposed: Vec<String> = Vec::new();
    for i in 0..system[0].len() {
        let column: String = system
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        transposed.push(column);
    }

    // Initialize a vector to hold final positions of round rocks
    let mut positions: Vec<usize> = Vec::new();

    // Iterate over each line and "move" round rocks to the left
    for line in &transposed {
        let line_chars: Vec<char> = line.chars().collect::<Vec<char>>();

        // Store the amount of empty space as number of '.' chars till a '#' which resets it
        let mut empty_space: usize = 0;
        for i in 0..line.len() {
            if line_chars[i] == '.' {
                empty_space += 1;
            }
            if line_chars[i] == '#' {
                empty_space = 0;
            }
            if line_chars[i] == 'O' {
                // If no empty space exists, keep it where it is, else push it to the left
                if empty_space == 0 {
                    positions.push(i);
                } else {
                    // Pushing a round rock to the left leaves the number of empty spaces unchanged
                    positions.push(i - empty_space);
                }
            }
        }
    }

    // Calculate the sum as inverse of distance
    for position in positions {
        sum += transposed[0].len() - position;
    }

    // Print final result
    println!("{:?}", sum);
}
