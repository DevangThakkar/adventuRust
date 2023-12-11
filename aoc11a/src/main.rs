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

    // Define dimensions of the grid
    let n_lines: usize = 140;

    // Define the multiplier
    let multiplier: usize = 1000000;

    // Initialize variable to hold the sum of added values
    let mut path_len_sum: usize = 0;

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Create vector to hold strings that had empty rows and columns replaced
    let mut lines_hori_fixed: Vec<String> = Vec::new();
    let mut lines_both_fixed: Vec<String> = Vec::new();

    // Create a counter of rows and columns to be multiplied instead of create large vectors
    let mut bad_rows: Vec<usize> = Vec::new();
    let mut bad_cols: Vec<usize> = Vec::new();

    // Iterate over each line in the file
    for i in 0..n_lines {
        // Iterate over lines and collect characters into a vector
        let line: String = lines[i].as_ref().expect("Retrieving failed").to_string();
        let uniq_line_chars: Vec<char> = line.chars().unique().collect();

        // Repeat lines with only .
        if uniq_line_chars.len() == 1 && uniq_line_chars[0] == '.' {
            // Refactor to fit a large multiplier
            bad_rows.push(i);
        }
        lines_hori_fixed.push(line.clone());
    }

    // Transpose vector of strings
    let mut transposed: Vec<String> = Vec::new();
    for i in 0..lines_hori_fixed[0].len() {
        let column: String = lines_hori_fixed
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        transposed.push(column);
    }

    // Iterate over each line in the file
    for i in 0..lines_hori_fixed[0].len() {
        // Iterate over lines and collect characters into a vector
        let line: String = transposed[i].clone();
        let uniq_line_chars: Vec<char> = line.chars().unique().collect();

        // Repeat lines with only .
        if uniq_line_chars.len() == 1 && uniq_line_chars[0] == '.' {
            // Refactor to fit a large multiplier
            bad_cols.push(i);
        }
        lines_both_fixed.push(line.clone());
    }

    // Create pairs of galaxies
    let mut positions: Vec<Vec<usize>> = Vec::new();
    for i in 0..lines_both_fixed.len() {
        let line: String = lines_both_fixed[i].clone();
        let line_chars: Vec<char> = line.chars().collect();
        for j in 0..lines_both_fixed[0].len() {
            if line_chars[j] == '#' {
                positions.push(vec![i, j]);
            }
        }
    }

    // Define function to calculate distance based on multiplier and number of bad rows or columns in between
    fn get_dist(start: usize, stop: usize, bad_pos: Vec<usize>, multiplier: usize) -> usize {
        let mut sum: usize = 0;
        for i in start..stop {
            if bad_pos.contains(&i) {
                sum += multiplier;
            } else {
                sum += 1;
            }
        }
        return sum;
    }

    // Find pairwise distances between galaxies
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let position_i: Vec<usize> = positions[i].clone();
            let position_j: Vec<usize> = positions[j].clone();

            // Calculate distance using manhattan distance and get_dist()
            let dist1: usize = if position_i[0] > position_j[0] {
                get_dist(position_j[0], position_i[0], bad_cols.clone(), multiplier)
            } else {
                get_dist(position_i[0], position_j[0], bad_cols.clone(), multiplier)
            };
            let dist2: usize = if position_i[1] > position_j[1] {
                get_dist(position_j[1], position_i[1], bad_rows.clone(), multiplier)
            } else {
                get_dist(position_i[1], position_j[1], bad_rows.clone(), multiplier)
            };

            // Add up distances in each direction
            path_len_sum += dist1 + dist2;
        }
    }
    // Print final sum
    println!("{:?}", path_len_sum);
}
