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

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Initialize a variable to keep track of the current line
    let mut counter: usize = 0;

    // Initialize a vector to count how many times a card was used
    let mut card_counter: Vec<u32> = vec![1; 192];

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Get all scratch cards after the colon
        let all_card_str: &str = line.split(":").nth(1).expect("Parse failed");

        // Get all winning scratch cards before the pipe
        let win_card_str_: &str = all_card_str.split("|").nth(0).expect("Parse failed");
        let win_card_str: String = win_card_str_.trim().replace("  ", " ");

        // Get all winning scratch cards into a vector
        let win_card_vec: Vec<u32> = win_card_str
            .split(" ")
            .map(|c| c.parse::<u32>().ok().unwrap())
            .collect();

        // Get all elf scratch cards after the pipe
        let elf_card_str_: &str = all_card_str.split("|").nth(1).expect("Parse failed");
        let elf_card_str: String = elf_card_str_.trim().replace("  ", " ");

        // Get all winning scratch cards into a vector
        let elf_card_vec: Vec<u32> = elf_card_str
            .split(" ")
            .map(|c| c.parse::<u32>().ok().unwrap())
            .collect();

        // Get overlap of winning cards and elf cards
        let overlap: Vec<_> = elf_card_vec
            .iter()
            .filter(|&c| win_card_vec.contains(c))
            .collect();

        // Get length of overlap for each card
        let overlap_len: usize = overlap.len().try_into().unwrap();
        for i in (counter + 1)..=(counter + overlap_len) {
            card_counter[i] += card_counter[counter];
        }

        // Increment the counter
        counter += 1;
    }
    // Print the final number of scratch cards
    let n_total: u32 = card_counter.iter().sum();
    println!("{:?}", n_total);
}
