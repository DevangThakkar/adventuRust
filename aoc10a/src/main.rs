// Import necessary modules and libraries
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

fn main() {
    // Define the path to the input file
    let file_path: &str = "input.txt";

    // Define dimensions of the grid
    let n_lines: usize = 140;

    // Initialize variable to hold the sum of added values
    let mut path_len: usize = 0;

    // Initialize a vector to hold the path
    let mut path: Vec<usize> = Vec::new();

    // Initialize a hash map to store direction mapping
    let mut dir_map: HashMap<usize, Vec<usize>> = HashMap::new();

    // Initialize line counter
    let mut counter: usize = 0;

    // Initialize start point
    let mut start: usize = 0;

    // Store the actual type of S
    let s_type: char = '|'; // HARDCODED COPOUT FOR PART 2

    // Read lines from the file and store them in a vector
    let lines_ = read_lines(file_path).expect("Reading failed");
    let lines: Vec<_> = lines_.collect();

    // Iterate over each line in the file
    for j in 0..n_lines {
        // Iterate over lines and collect characters into a vector
        let line: String = lines[j].as_ref().expect("Retrieving failed").to_string();
        let line_chars: Vec<char> = line.chars().collect();

        // Iterate over characters in line
        for i in 0..n_lines {
            // Initialize two destinations per square
            let (mut map1, mut map2): (usize, usize);

            // Define destinations if current square is F
            if line_chars[i] == 'F' {
                if i == n_lines - 1 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i + 1;
                }
                if counter == n_lines - 1 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i + n_lines;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is 7
            if line_chars[i] == '7' {
                if i == 0 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i - 1;
                }
                if counter == n_lines - 1 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i + n_lines;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is J
            if line_chars[i] == 'J' {
                if i == 0 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i - 1;
                }
                if counter == 0 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i - n_lines;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is L
            if line_chars[i] == 'L' {
                if i == n_lines - 1 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i + 1;
                }
                if counter == 0 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i - n_lines;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is -
            if line_chars[i] == '-' {
                if i == n_lines - 1 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i + 1;
                }
                if i == 0 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i - 1;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is |
            if line_chars[i] == '|' {
                if counter == n_lines - 1 {
                    map1 = 100000;
                } else {
                    map1 = counter * n_lines + i + n_lines;
                }
                if counter == 0 {
                    map2 = 100000;
                } else {
                    map2 = counter * n_lines + i - n_lines;
                }
                dir_map.insert(counter * n_lines + i, vec![map1, map2]);
            }

            // Define destinations if current square is S: SPECIAL
            // We don't know the destinations if current square is S, it's defined by others
            // Requires knowledge of the previous and next line (if they exist)
            if line_chars[i] == 'S' {
                start = counter * n_lines + i;
                let mut start_map: Vec<usize> = Vec::new();
                if i != 0
                    && (line_chars[i - 1] == '-'
                        || line_chars[i - 1] == 'L'
                        || line_chars[i - 1] == 'F')
                {
                    start_map.push(counter * n_lines + i - 1);
                }
                if i != n_lines - 1
                    && (line_chars[i + 1] == '-'
                        || line_chars[i + 1] == 'J'
                        || line_chars[i + 1] == '7')
                {
                    start_map.push(counter * n_lines + i + 1);
                }
                if counter != 0 {
                    let previous: String = lines[j - 1]
                        .as_ref()
                        .expect("Retrieving failed")
                        .to_string();
                    let previous_chars: Vec<char> = previous.chars().collect();
                    if previous_chars[i] == '|'
                        || previous_chars[i] == 'L'
                        || previous_chars[i] == 'J'
                    {
                        start_map.push(counter * n_lines + i - n_lines);
                    }
                }
                if counter != n_lines - 1 {
                    let next: String = lines[j + 1]
                        .as_ref()
                        .expect("Retrieving failed")
                        .to_string();
                    let next_chars: Vec<char> = next.chars().collect();
                    if next_chars[i] == '|' || next_chars[i] == 'L' || next_chars[i] == 'J' {
                        start_map.push(counter * n_lines + i + n_lines);
                    }
                }
                dir_map.insert(counter * n_lines + i, start_map);
            }
        }
        // Increment line counter
        counter += 1;
    }

    // Initialize variables to hold current and next squares
    let mut current: usize = start.clone();
    let mut next: usize;

    // Loop till you reach the source square again
    loop {
        // Get destinations for the current square
        let destinations = dir_map.get(&current).expect("Retrieving failed").clone();

        // Special case for the starting square
        if path_len == 0 {
            dir_map.insert(current.clone(), vec![100000, 100000]);
            next = destinations[0];
        } else {
            if destinations[0] != 100000 {
                next = destinations[0];
            } else if destinations[1] != 100000 {
                next = destinations[1];
            } else {
                break;
            }
        }
        path_len += 1;
        path.push(next);

        // Get destinations for the next square
        let next_destinations = dir_map.get(&next).expect("Retrieving failed").clone();
        if next_destinations[0] == current {
            dir_map.insert(next.clone(), vec![100000, next_destinations[1]]);
        } else {
            dir_map.insert(next.clone(), vec![next_destinations[0], 100000]);
        }
        current = next;
    }

    // Initialize variable to hold counts of points inside
    let mut counts = 0;
    for i in 0..n_lines {
        // Initialize flip counter to 0
        let mut flip: usize = 0;

        // Initialize starting corner
        let mut corner: char = '.';

        for j in 0..n_lines {
            let counter = i * n_lines + j;

            // Iterate over lines and collect characters into a vector
            let line: String = lines[i].as_ref().expect("Retrieving failed").to_string();
            let mut line_chars: Vec<char> = line.chars().collect();

            // Insert hardcoded value for S
            if line_chars[j] == 'S' {
                line_chars[j] = s_type;
            }

            // If path contains current square, see if we need to add a flip or not
            if path.contains(&counter) {
                // - does not affect flipping
                if line_chars[j] == '-' {
                    continue;
                }
                // J only affects flipping if preceded by F (excluding -), else it is a hairpin
                if line_chars[j] == 'J' {
                    if corner == 'F' {
                        flip += 1;
                    }
                    corner = 'J';
                }

                // 7 only affects flipping if preceded by L (excluding -), else it is a hairpin
                if line_chars[j] == '7' {
                    if corner == 'L' {
                        flip += 1;
                    }
                    corner = '7';
                }

                // L doesn't affect flipping by itself but we need to remember it for 7
                if line_chars[j] == 'L' {
                    corner = 'L';
                }

                // F doesn't affect flipping by itself but we need to remember it for J
                if line_chars[j] == 'F' {
                    corner = 'F';
                }

                // | is a flip
                if line_chars[j] == '|' {
                    flip += 1;
                }
            }
            // If there have been odd number of flips so far, add to count
            else {
                if flip % 2 == 1 {
                    counts += 1;
                }
            }
        }
    }
    // Print final result
    println!("{:?}", counts);
}
