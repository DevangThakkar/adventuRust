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

// Define a function that transposes the matrix - required for N/S
fn transpose(string_vec: Vec<String>) -> Vec<String> {
    let mut transposed: Vec<String> = Vec::new();
    for i in 0..string_vec[0].len() {
        let column: String = string_vec
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        transposed.push(column);
    }

    return transposed;
}

// Define a function that reflects the matrix - required for E/S
fn reflect(string_vec: Vec<String>) -> Vec<String> {
    let reflected: Vec<String> = string_vec
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect();
    return reflected;
}

// Define a function that shifts the rocks in the direction needed
fn shift(system: Vec<String>, direction: &str) -> Vec<String> {
    // Initialize vector to store the result prior to re-transposing/re-reflecting
    let mut result_vec: Vec<String> = Vec::new();

    // Edit matrix based on direction provided
    let new_system: Vec<String>;
    if direction == "north" {
        new_system = transpose(system.clone());
    } else if direction == "west" {
        new_system = system.clone();
    } else if direction == "south" {
        new_system = reflect(transpose(system.clone()));
    } else {
        new_system = reflect(system.clone());
    }

    // Iterate over each line and "move" round rocks to the left
    for line in &new_system {
        let line_chars: Vec<char> = line.chars().collect::<Vec<char>>();

        // Initialize a new vector to store the shifted line
        let mut new_line: Vec<char> = Vec::new();

        // Initialize a counter that defines the limit to which round rocks can slide
        let mut start_pos: usize = 0;

        // Initialize a flag variable to distinguish between '...O' and '#..O'
        // The start_pos is zero in both cases but in the second one, we want it after the #
        let mut found: usize = 0;
        for i in 0..line.len() {
            if line_chars[i] == '.' {
                new_line.push('.');
            }
            if line_chars[i] == '#' {
                new_line.push('#');
                start_pos = i;
                found = 1;
            }
            if line_chars[i] == 'O' {
                if start_pos == 0 && found == 0 {
                    new_line.insert(0, 'O');
                } else {
                    new_line.insert(start_pos + 1, 'O');
                }
            }
        }
        let char_string: String = new_line.iter().cloned().collect();
        result_vec.push(char_string);
    }

    // Initialize vector to store the result post re-transposing/re-reflecting
    let new_result_vec: Vec<String>;
    if direction == "north" {
        new_result_vec = transpose(result_vec.clone());
    } else if direction == "west" {
        new_result_vec = result_vec.clone();
    } else if direction == "south" {
        new_result_vec = transpose(reflect(result_vec.clone())); // Reverse order of initial transform
    } else {
        new_result_vec = reflect(result_vec.clone());
    }

    // Return final system after returing to original orientation
    return new_result_vec;
}

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Initialize vector of Strings to store the system in the input
    let mut system: Vec<String> = Vec::new();

    // Define number of iterations
    let n_iter = 1000000000;

    // Initialize variable to store final sum
    let mut sum: usize = 0;

    // Iterate over each line in the file to read notes
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");
        system.push(line);
    }

    // Initialize new system as a copy of the system and a history to store past systems
    let mut new_system = system.clone();
    let mut history: Vec<Vec<String>> = Vec::new();

    // Initialize variables to identify loop specifications
    let mut previous: usize = n_iter + 1;
    let mut interval: usize = n_iter + 1;

    // Iterate till a loop is found
    for i in 0..n_iter {
        // Shift stones in all directions per cycle
        let result_north: Vec<String> = shift(new_system, "north");
        let result_west: Vec<String> = shift(result_north, "west");
        let result_south: Vec<String> = shift(result_west, "south");
        let result_east: Vec<String> = shift(result_south, "east");
        new_system = result_east.clone();

        // Check if current system was found before
        if history.contains(&new_system) {
            previous = history
                .iter()
                .position(|s| s == &new_system)
                .expect("Retrieving failed");

            // Determine loop length
            interval = i - previous;
            break;
        }
        history.push(new_system.clone());
    }

    // Identify the location in the cycle that the final value will be at
    let new_limit: usize = ((n_iter - 1) - previous) % interval;

    // Obtain system for final value
    let final_system: Vec<String> = history[previous + new_limit].clone();

    // Calculate total load on north beams
    for i in 0..final_system.len() {
        sum += final_system[i].chars().filter(|&c| c == 'O').count() * (final_system.len() - i);
    }

    // Print final result
    println!("{:?}", sum);
}
