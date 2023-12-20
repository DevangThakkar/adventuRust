// Import necessary modules and libraries
use std::fs::read_to_string;

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Read the only line from the file
    let lines_ = read_to_string(file_path).expect("Reading failed");

    // Split the line into steps separated by a comma
    let steps: Vec<&str> = lines_.trim().split(",").collect();

    // Initialize a variable to hold the final sum of hashes
    let mut sum: usize = 0;

    // Iterate over each step and obtain its corresponding hash
    for step in steps {
        // Initialize a variable to hold value for each step
        let mut step_val: usize = 0;

        // Use the hash technique to obtain value for each step
        for i in 0..step.len() {
            let character: char = step.chars().nth(i).expect("Retrieving failed");
            let ascii: usize = character as usize;
            step_val += ascii;
            step_val *= 17;
            step_val %= 256;
        }
        sum += step_val;
    }

    println!("{:?}", sum);
}
