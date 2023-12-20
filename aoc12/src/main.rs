// Import necessary modules and libraries
use rayon::prelude::*;
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

// Define function to get the number of possible arrangements
fn get_ways(springs: &str, counts: Vec<usize>) -> usize {
    // Initialize variable to store possible counts
    let mut n_ways: usize = 0;

    // Create a String from springs
    let mut springs_string = springs.to_string();

    // Use char_indices to get (index, char), filter on char == '?' and then use index
    let q_mark_positions: Vec<usize> = springs_string
        .char_indices()
        .filter(|(_index, character)| *character == '?')
        .map(|(index, _character)| index)
        .collect();

    // .len() gives usize but .pow() can't use usize
    for i in 0..2_usize.pow(q_mark_positions.len() as u32) {
        // Not sure why & is needed for pos here
        for (j, &pos) in q_mark_positions.iter().enumerate() {
            let new = if (i / (2_usize.pow(j as u32))) % 2 == 0 {
                '.'
            } else {
                '#'
            };
            springs_string.replace_range(pos..pos + 1, &new.to_string());
        }
        let springs_char: Vec<char> = springs_string.chars().collect();
        let mut new_counts: Vec<usize> = Vec::new();
        let mut yes: usize = 0;
        for j in 0..springs_char.len() {
            if springs_char[j] == '#' {
                yes += 1;
            } else {
                if yes > 0 {
                    new_counts.push(yes);
                    yes = 0;
                }
            }
        }
        if yes > 0 {
            new_counts.push(yes);
        }
        // println!("{:?} {:?}", springs_char, new_counts);
        if new_counts.len() == counts.len() {
            if new_counts.iter().zip(&counts).all(|(a, b)| a == b) {
                n_ways += 1;
            }
        }
    }
    // n_ways = if n_ways > 0 { n_ways } else { 1 };
    return n_ways;
}

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Use Rayon's par_iter to parallelize the processing of lines
    let sum_ways: usize = lines
        .par_iter()
        .map(|line| {
            let line: String = line.as_ref().expect("Retrieving failed").to_string();
            let line_split: Vec<&str> = line.split(" ").collect();
            let springs: String = line_split[0].to_string();
            let counts: Vec<usize> = line_split[1]
                .split(",")
                .map(|c| c.parse::<usize>().ok().unwrap())
                .collect();
            let ways: usize = get_ways(&springs, counts);
            ways
        })
        .sum();
    // Print final result
    println!("{:?}", sum_ways);
}
