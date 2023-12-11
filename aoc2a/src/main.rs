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
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Define regular expressions for parsing the input
    let re: Regex = Regex::new(r"^(.*?)(: )(.*)$").unwrap(); // Matches lines of the form "text : text"
    let re_red: Regex = Regex::new(r"([0-9]+) red").unwrap(); // Matches numbers followed by " red"
    let re_green: Regex = Regex::new(r"([0-9]+) green").unwrap(); // Matches numbers followed by " green"
    let re_blue: Regex = Regex::new(r"([0-9]+) blue").unwrap(); // Matches numbers followed by " blue"

    // Initialize a variable to store the sum of powers
    let mut power_sum: u32 = 0;

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Extract the third capturing group from the regex match
        let processed: String = re
            .captures(&line)
            .expect("Regex failed")
            .get(3)
            .expect("Regex failed")
            .as_str()
            .replace(";", "")
            .replace(",", "");

        // Initialize variables to store maximum values for red, green, and blue
        let (mut red_max, mut green_max, mut blue_max): (u32, u32, u32) = (0, 0, 0);

        // Iterate over captures of red values in the processed text
        for capture in re_red.captures_iter(&processed) {
            // Parse the captured red count as a u32
            let red_count: u32 = capture
                .get(1)
                .expect("Parsing red failed")
                .as_str()
                .parse::<u32>()
                .expect("Parsing read failed");
            // Update red_max if the current red_count is greater
            if red_count > red_max {
                red_max = red_count;
            }
        }
        // Iterate over captures of green values in the processed text
        for capture in re_green.captures_iter(&processed) {
            // Parse the captured green count as a u32
            let green_count: u32 = capture
                .get(1)
                .expect("Parsing green failed")
                .as_str()
                .parse::<u32>()
                .expect("Parsing read failed");
            // Update green_max if the current green_count is greater
            if green_count > green_max {
                green_max = green_count;
            }
        }
        // Iterate over captures of green values in the processed text
        for capture in re_blue.captures_iter(&processed) {
            // Parse the captured green count as a u32
            let blue_count: u32 = capture
                .get(1)
                .expect("Parsing blue failed")
                .as_str()
                .parse::<u32>()
                .expect("Parsing read failed");
            // Update green_max if the current green_count is greater
            if blue_count > blue_max {
                blue_max = blue_count;
            }
        }
        // Update the power_sum with the product of red_max, green_max, and blue_max
        power_sum += red_max * green_max * blue_max;
    }
    // Print the final computed power_sum
    println!("{:?}", power_sum);
}
