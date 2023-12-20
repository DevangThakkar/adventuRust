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

// Define a function that takes in a note and returns a vector with the mirror index if present
fn get_intersect(note: &Vec<String>) -> Vec<usize> {
    //
    let mut intersection: Vec<usize> = (1..10000).collect();

    // Iterate over each line of the note
    for i in 0..note.len() {
        let mut possible_indices: Vec<usize> = Vec::new();

        // Create a new note which makes ABCD into A-B-C-D
        // The filter removes empty strings on the ends, preventing -A-B-C-D-
        let mod_note = note[i]
            .split("")
            .filter(|&c| !c.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        // Iterate over the characters in each row of the modified note A-B-C-D
        for j in 0..mod_note.len() {
            // We don't care about the even positions, i.e. hyphens
            if j % 2 == 1 {
                // Initialize a counter variable that checks if every line is a palindrome
                let mut choose: usize = 0;

                // Iterate over all lengths of strings centered on the character indexed by j
                for k in 0..=j.min(mod_note.len() - j - 1) {
                    // Define forward and reverse strings
                    let forward_vec = mod_note[j - k..=j + k].chars().collect::<Vec<_>>();
                    let reverse_vec = mod_note[j - k..=j + k].chars().rev().collect::<Vec<_>>();

                    // If forward and reverse strings are the same, make choose to 1 if it is not 1 already
                    if forward_vec == reverse_vec {
                        if forward_vec.len() > 1 {
                            choose = 1;
                        }
                    }
                    // If any string is not a palindrome, choose becomes 0
                    else {
                        choose *= 0;
                    }
                }
                // If all strings are palindromes, push this index as a possible index
                if choose == 1 {
                    possible_indices.push(j);
                }
            }
        }
        // Calculate intersection of possible indices with intersection
        // Intersection is 0..1e5 initially so the intersection is equal to possible_indices the first time
        intersection = possible_indices
            .iter()
            .cloned()
            .filter(|&x| intersection.contains(&x))
            .collect();
    }
    return intersection;
}

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Initialize vector of vector of strings to store all notes
    let mut notes_vec: Vec<Vec<String>> = Vec::new();

    // Initialize vector of Strings to store a note
    let mut note: Vec<String> = Vec::new();

    // Initialize variable to store final sum
    let mut sum: usize = 0;

    // Iterate over each line in the file to read notes
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Add new note on encountering a new line and reset note
        if line == "" {
            notes_vec.push(note);
            note = Vec::new();
        } else {
            note.push(line);
        }
    }
    // Push final note
    notes_vec.push(note);

    // Iterate over notes in notes_vec and get mirror index
    for note in &notes_vec {
        // Get mirror index for the original note, i.e. split within row
        let hori_index_vec = get_intersect(note);

        // Initialize hori_index to a high value to determine when it is not found
        let mut hori_index: usize = 100000;

        // Make sure there is an element in the returned value
        if hori_index_vec.len() == 1 {
            hori_index = (hori_index_vec[0] + 1) / 2;
        }

        // Transpose vector of strings to look at split within column
        let mut transposed: Vec<String> = Vec::new();
        for i in 0..note[0].len() {
            let column: String = note
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect();
            transposed.push(column);
        }

        // Get mirror index for the original note, i.e. split within row
        let vert_index_vec = get_intersect(&transposed);

        // Initialize hori_index to a high value to determine when it is not found
        let mut vert_index: usize = 100000;

        // Make sure there is an element in the returned value
        if vert_index_vec.len() == 1 {
            vert_index = (vert_index_vec[0] + 1) / 2;
        }

        // Figure out where the mirror was found
        if hori_index != 100000 {
            sum += hori_index;
        } else {
            sum += vert_index * 100;
        }
    }

    // Print final result
    println!("{:?}", sum);
}
