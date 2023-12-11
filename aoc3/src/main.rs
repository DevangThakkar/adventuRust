// Import necessary modules and libraries
use regex::Regex;
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
    // Specify the file path and the number of lines in the file
    let file_path: &str = "input.txt";
    let n_lines: u32 = 140;

    // Define a regex pattern to match digits
    let re: Regex = Regex::new(r"\d+").unwrap();

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Initialize a variable to store the sum of numbers of interest
    let mut sum: u32 = 0;

    // Create strings to represent the padding for the first and last lines
    let previous_: String = std::iter::repeat('.').take(140).collect();
    let next_: String = previous_.clone();

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
        let next__: &str = if i < (n_lines - 1) {
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
            let substring: &str = &digits.as_str();

            // Extract the neighboring characters around the matched digits
            let neighbors: String = format!(
                "{}{}{}{}",
                &current[start_position - 1..start_position],
                &previous[start_position - 1..=end_position],
                &next[start_position - 1..=end_position],
                &current[end_position..=end_position]
            );

            // Replace digits with dots in the neighbor string
            let neighbors_no_num: String = neighbors
                .chars()
                .map(|c| if c.is_digit(10) { '.' } else { c })
                .collect();

            // Count the number of dots in the neighbor string
            let dot_count: usize = neighbors_no_num.chars().filter(|&c| c == '.').count();

            // Check if the number of characters in the neighbor string is not equal to the dot count
            if neighbors_no_num.chars().count() != dot_count {
                sum += substring.parse::<u32>().expect("Parsing failed");
            }
        }
    }
    // Print the final sum of numbers of interest
    println!("{:?}", sum);
}
