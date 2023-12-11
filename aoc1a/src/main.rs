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

// Function to extract the first digit from a string using regex
fn get_first(input_str: &str, dir: &str) -> u32 {
    // Define regex patterns for parsing numbers in forward and reverse directions
    let mut re =
        Regex::new(r"^(.*?)([0-9]|one|two|three|four|five|six|seven|eight|nine)(.*)$").unwrap();
    if dir == "reverse" {
        re =
            Regex::new(r"^(.*?)([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)(.*)$").unwrap();
    }

    // Define a mapping from lettered numbers to digits
    let digit_mapping: HashMap<String, String> = HashMap::from([
        (String::from("one"), String::from("1")),
        (String::from("eno"), String::from("1")),
        (String::from("two"), String::from("2")),
        (String::from("owt"), String::from("2")),
        (String::from("three"), String::from("3")),
        (String::from("eerht"), String::from("3")),
        (String::from("four"), String::from("4")),
        (String::from("ruof"), String::from("4")),
        (String::from("five"), String::from("5")),
        (String::from("evif"), String::from("5")),
        (String::from("six"), String::from("6")),
        (String::from("xis"), String::from("6")),
        (String::from("seven"), String::from("7")),
        (String::from("neves"), String::from("7")),
        (String::from("eight"), String::from("8")),
        (String::from("thgie"), String::from("8")),
        (String::from("nine"), String::from("9")),
        (String::from("enin"), String::from("9")),
    ]);

    let mut digit: u32 = 0;

    // Check if the regex pattern matches the input string
    if let Some(captures) = re.captures(input_str) {
        // Check if the second capture group (the digit) exists
        if let Some(digit_match) = captures.get(2) {
            // Use the digit mapping if available, otherwise use the captured string
            if let Some(digit_) = digit_mapping.get(digit_match.as_str()) {
                digit = digit_.chars().next().unwrap().to_digit(10).unwrap();
            } else {
                digit = digit_match
                    .as_str()
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
            }
        }
    }
    // Return the extracted digit
    digit
}

fn main() {
    let file_path: &str = "input.txt";
    let mut sum: u32 = 0;

    // Attempt to read lines from the file
    if let Ok(lines) = read_lines(file_path) {
        // Iterate over each line in the file
        for unchecked_line in lines {
            // Attempt to unwrap the line
            if let Ok(line) = unchecked_line {
                // Get the first digit from the original line
                let first_digit: u32 = get_first(&line, "forward");

                // Reverse the line and get the first digit from the reversed line
                let reversed_line: String = line.chars().rev().collect();
                let last_digit: u32 = get_first(&reversed_line, "reverse");

                // Combine the first and last digits into a number and add it to the sum
                let number: u32 = first_digit * 10 + last_digit;
                sum += number;
            }
        }
    }
    // Print the final sum
    println!("{:?}", sum);
}
