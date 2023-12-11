// Import necessary modules and libraries
use regex::Regex;
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

fn find_char_indices(input_string: &str, target_char: char) -> Vec<usize> {
    // Find the indices where the target_char is found in the input_string
    input_string
        .char_indices()
        .filter(|&(_, c)| c == target_char)
        .map(|(index, _)| index)
        .collect()
}

fn main() {
    // Specify the file path and the number of lines in the file
    let file_path: &str = "input.txt";
    let n_lines: u32 = 140;

    // Define a regex pattern to match digits
    let re: Regex = Regex::new(r"\d+").unwrap();

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Initialize a variable to store the sum of gear ratios
    let mut sum: u32 = 0;

    // Create strings to represent the padding for the first and last lines
    let previous_: String = std::iter::repeat('.').take(140).collect();
    let next_: String = previous_.clone();

    // Create a hash map to store the numbers next to stars
    let mut star_map: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over each line in the specified range
    for i in 0..n_lines {
        // Convert u32 index to usize
        let i_usize: usize = i as usize;

        // Create strings representing the previous line with padding dots
        let previous__: &str = if i > 0 {
            lines[i_usize - 1].as_ref().expect("Iterating failed")
        } else {
            &previous_
        };
        let previous: String = format!(".{}.", previous__);

        // Create strings representing the current line with padding dots
        let current_: &str = lines[i_usize].as_ref().expect("Iterating failed");
        let current: String = format!(".{}.", current_);

        // Create strings representing the next line with padding dots
        let next__: &str = if i < n_lines - 1 {
            lines[i_usize + 1].as_ref().expect("Iterating failed")
        } else {
            &next_
        };
        let next: String = format!(".{}.", next__);

        // Iterate over captures of digits in the current line
        for capture in re.captures_iter(&current) {
            // Get the matched digits and their positions
            let digits = capture.get(0).expect("Regex failed");
            let start_position: usize = digits.start();
            let end_position: usize = digits.end();
            let substring: &str = digits.as_str();

            // Add entry to hash map if neighbor to the left is a star
            let neigh1: &str = &current[start_position - 1..start_position];
            if neigh1 == "*" {
                star_map
                    .entry(format!("{}:{}", i, start_position - 1))
                    .or_insert(Vec::new())
                    .push(substring.to_string().clone());
            }
            // Add entry to hash map if any neighbor in the previous line is a star
            let neigh2: &str = &previous[start_position - 1..=end_position];
            let indices2: Vec<usize> = find_char_indices(neigh2, '*');
            for index in indices2 {
                star_map
                    .entry(format!("{}:{}", i - 1, start_position - 1 + index))
                    .or_insert(Vec::new())
                    .push(substring.to_string().clone());
            }
            // Add entry to hash map if any neighbor in the next line is a star
            let neigh3: &str = &next[start_position - 1..=end_position];
            let indices3: Vec<usize> = find_char_indices(neigh3, '*');
            for index in indices3 {
                star_map
                    .entry(format!("{}:{}", i + 1, start_position - 1 + index))
                    .or_insert(Vec::new())
                    .push(substring.to_string().clone());
            }
            // Add entry to hash map if neighbor to the right is a star
            let neigh4: &str = &current[end_position..=end_position];
            if neigh4 == "*" {
                star_map
                    .entry(format!("{}:{}", i, end_position))
                    .or_insert(Vec::new())
                    .push(substring.to_string().clone());
            }
        }
    }
    // Iterate over values in the hash map and calculate gear ratio
    for key in &star_map {
        let val = key.1;
        // Add gear ratio if a star has two neighboring numbers
        if val.len() == 2 {
            let num1 = val[0].parse::<u32>().expect("Parsing failed");
            let num2 = val[1].parse::<u32>().expect("Parsing failed");
            sum += num1 * num2;
        }
    }
    // Print the final sum of gear ratios
    println!("{:?}", sum);
}
