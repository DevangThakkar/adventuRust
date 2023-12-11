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

    // Define the allowed counts for red, green, and blue
    let allowed_red_count: u32 = 12;
    let allowed_green_count: u32 = 13;
    let allowed_blue_count: u32 = 14;

    // Initialize a variable to store the sum
    let mut sum: u32 = 0;

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Iterate over each line in the file
    for unchecked_line in lines {
        let game_id: u32;

        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Get the game_id from the second item when split by whitespace and remove colon
        let game_id_str: &str = line.split_whitespace().nth(1).expect("Parse failed");
        game_id = game_id_str
            .trim_end_matches(':')
            .parse::<u32>()
            .expect("Converting failed");

        // Extract the third capturing group from the regex match
        let processed: String = re
            .captures(&line)
            .expect("Regex failed")
            .get(3)
            .expect("Regex failed")
            .as_str()
            .replace(";", "")
            .replace(",", "");

        // Initialize variables to store flag values for failure for red, green, and blue
        let (mut red_failed, mut green_failed, mut blue_failed): (u32, u32, u32) = (0, 0, 0);

        // Iterate over captures of red values in the processed text
        for capture in re_red.captures_iter(&processed) {
            // Parse the captured red count as a u32
            let red_count: u32 = capture
                .get(1)
                .expect("Parsing red failed")
                .as_str()
                .parse::<u32>()
                .expect("Parsing read failed");
            // Update red_failed if the current red_count is greater than allowed_red_count
            if red_count > allowed_red_count {
                red_failed = 1;
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
            // Update green_failed if the current green_count is greater than allowed_green_count
            if green_count > allowed_green_count {
                green_failed = 1;
            }
        }

        // Iterate over captures of blue values in the processed text
        for capture in re_blue.captures_iter(&processed) {
            // Parse the captured blue count as a u32
            let blue_count: u32 = capture
                .get(1)
                .expect("Parsing blue failed")
                .as_str()
                .parse::<u32>()
                .expect("Parsing read failed");
            // Update blue_failed if the current blue_count is greater than allowed_blue_count
            if blue_count > allowed_blue_count {
                blue_failed = 1;
            }
        }

        // Update the sum if all three pass
        if red_failed + green_failed + blue_failed == 0 {
            sum += game_id;
        }
    }

    // Print the final computed sum
    println!("{:?}", sum);
}
