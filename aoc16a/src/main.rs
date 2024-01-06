// Import necessary modules and libraries
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

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

    // Initialize a hash map that stores the squares that can be entered from a given square
    // Squares are defined not just by coordinates but also by the direction of the ray
    // While the key is a single square, the value is a vector to accomodate splitters
    let mut directions: HashMap<String, Vec<String>> = HashMap::new();

    // Row_count and col_count define the i and j for the matrix
    let mut row_count = 0;

    // Initialize an atomic variable to hold the maximum path length over parallel runs
    let max_len: AtomicU64 = AtomicU64::new(0);

    // Iterate over each line in the file
    for unchecked_line in lines {
        row_count += 1;

        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");
        let line_chars: Vec<char> = line.chars().collect::<Vec<char>>();

        // Row_count and col_count define the i and j for the matrix
        let mut col_count = 0;

        // Iterate over each row and fill the directions hashmap
        for symbol in line_chars.iter() {
            col_count += 1;

            // '.' means the ray goes in the same direction as before
            if symbol == &'.' {
                directions.insert(
                    format!("{},{},L", row_count, col_count),
                    vec![format!("{},{},L", row_count, col_count - 1)],
                );
                directions.insert(
                    format!("{},{},R", row_count, col_count),
                    vec![format!("{},{},R", row_count, col_count + 1)],
                );
                directions.insert(
                    format!("{},{},U", row_count, col_count),
                    vec![format!("{},{},U", row_count - 1, col_count)],
                );
                directions.insert(
                    format!("{},{},D", row_count, col_count),
                    vec![format!("{},{},D", row_count + 1, col_count)],
                );
            }
            // '\' means L->U/U->L and R->D/D->R
            if symbol == &'\\' {
                directions.insert(
                    format!("{},{},L", row_count, col_count),
                    vec![format!("{},{},U", row_count - 1, col_count)],
                );
                directions.insert(
                    format!("{},{},R", row_count, col_count),
                    vec![format!("{},{},D", row_count + 1, col_count)],
                );
                directions.insert(
                    format!("{},{},U", row_count, col_count),
                    vec![format!("{},{},L", row_count, col_count - 1)],
                );
                directions.insert(
                    format!("{},{},D", row_count, col_count),
                    vec![format!("{},{},R", row_count, col_count + 1)],
                );
            }
            // '/' means L->D/D->L and R->U/U->R
            if symbol == &'/' {
                directions.insert(
                    format!("{},{},L", row_count, col_count),
                    vec![format!("{},{},D", row_count + 1, col_count)],
                );
                directions.insert(
                    format!("{},{},R", row_count, col_count),
                    vec![format!("{},{},U", row_count - 1, col_count)],
                );
                directions.insert(
                    format!("{},{},U", row_count, col_count),
                    vec![format!("{},{},R", row_count, col_count + 1)],
                );
                directions.insert(
                    format!("{},{},D", row_count, col_count),
                    vec![format!("{},{},L", row_count, col_count - 1)],
                );
            }
            // '|' means vertical goes in the same direction and L/R->U+D
            if symbol == &'|' {
                directions.insert(
                    format!("{},{},L", row_count, col_count),
                    vec![
                        format!("{},{},U", row_count - 1, col_count),
                        format!("{},{},D", row_count + 1, col_count),
                    ],
                );
                directions.insert(
                    format!("{},{},R", row_count, col_count),
                    vec![
                        format!("{},{},D", row_count + 1, col_count),
                        format!("{},{},U", row_count - 1, col_count),
                    ],
                );
                directions.insert(
                    format!("{},{},U", row_count, col_count),
                    vec![format!("{},{},U", row_count - 1, col_count)],
                );
                directions.insert(
                    format!("{},{},D", row_count, col_count),
                    vec![format!("{},{},D", row_count + 1, col_count)],
                );
            }
            // '-' means horizontal goes in the same direction and U/D->L+R
            if symbol == &'-' {
                directions.insert(
                    format!("{},{},L", row_count, col_count),
                    vec![format!("{},{},L", row_count, col_count - 1)],
                );
                directions.insert(
                    format!("{},{},R", row_count, col_count),
                    vec![format!("{},{},R", row_count, col_count + 1)],
                );
                directions.insert(
                    format!("{},{},U", row_count, col_count),
                    vec![
                        format!("{},{},L", row_count, col_count - 1),
                        format!("{},{},R", row_count, col_count + 1),
                    ],
                );
                directions.insert(
                    format!("{},{},D", row_count, col_count),
                    vec![
                        format!("{},{},R", row_count, col_count + 1),
                        format!("{},{},L", row_count, col_count - 1),
                    ],
                );
            }
        }
    }

    // Complicated way to get matrix size + 1 in string form
    let binding = (row_count + 1).to_string();
    let limit: &str = binding.as_str();

    // Unlike last time, we now want to look at all boundary entry points
    let mut all_entry_points: Vec<String> = Vec::new();

    // Create all entry points
    for i in 1..=row_count {
        let element_left: String = format!("{},1,R", i);
        let element_top: String = format!("1,{},D", i);
        let element_right: String = format!("{},{},L", row_count + 1 - i, row_count);
        let element_bottom: String = format!("{},{},U", row_count, row_count + 1 - i);

        all_entry_points.push(element_left);
        all_entry_points.push(element_top);
        all_entry_points.push(element_right);
        all_entry_points.push(element_bottom);
    }

    // Use into_par_iter for multithreading
    all_entry_points.into_par_iter().for_each(|square| {
        // Initialize vectors that hold visited places and places to visit
        let mut visited: Vec<String> = Vec::new();
        let mut to_visit: Vec<String> = Vec::new();

        // Push starting square to visited and to to_visit
        visited.push(square.clone());
        to_visit.push(square.clone());

        // Loop till you have places to visit
        loop {
            // Break if no more places left to visit
            if to_visit.len() == 0 {
                break;
            }

            // Iterate over all squares yet to be visited.
            let mut next_to_visit: Vec<String> = to_visit.clone();
            for place in &to_visit {
                let next_position_arr: Vec<String> =
                    directions.get(place).expect("Redirecting failed").to_vec();

                // For each position to visit, look at the destination(s)
                for item in next_position_arr {
                    // If already visited, skip
                    if visited.contains(&item) {
                        continue;
                    }
                    // If already set to visit, skip
                    if to_visit.contains(&item) {
                        continue;
                    }
                    let item_split: Vec<&str> = item.split(",").collect();
                    // If reached left or top boundary, skip
                    if item_split[0] == "0" || item_split[1] == "0" {
                        continue;
                    }
                    // If reached right or bottom boundary, skip
                    if item_split[0] == limit || item_split[1] == limit {
                        continue;
                    }
                    next_to_visit.push(item.clone());
                }

                // If not already visited, mark as visited
                if !visited.contains(&place) {
                    visited.push(place.clone());
                }
                // Remove duplicates
                next_to_visit.retain(|x| x != place);
            }
            to_visit = next_to_visit.clone();
        }

        // Define locations as squares, stripped to keep only the coordinates (1,1,R -> 1,1)
        let locations: Vec<String> = visited
            .iter()
            .map(|s| s.split(',').take(2).collect::<Vec<&str>>().join(","))
            .collect();
        // Locations has duplicates (e.g. 1,1,R and 1,1,L are both 1,1)
        let unique_locations: Vec<String> = locations
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // Store max path length across parallel runs
        let length: u64 = unique_locations.len() as u64;
        max_len.fetch_max(length, Ordering::Relaxed);
    });

    // Obtain max path length from parallel runs
    let max_len_value: u64 = max_len.load(Ordering::Relaxed);
    // Print final answer
    println!("{:?}", max_len_value);
}
