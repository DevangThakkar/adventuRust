// Import necessary modules and libraries
use itertools::Itertools;
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

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Initialize a vector to hold hands
    let mut hands: Vec<String> = Vec::new();

    // Initialize a hash map to store hands to bets mapping
    let mut bet_map: HashMap<String, usize> = HashMap::new();

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Split line into two parts around the white space
        let line_split: Vec<&str> = line.split(" ").collect();

        // Get the hand of cards
        let hand: &str = line_split.get(0).expect("Parsing failed");

        // Push individual hands to the vector
        hands.push(hand.to_string());

        // Get the bet amount
        let bet: usize = line_split
            .get(1)
            .expect("Parsing failed")
            .parse::<usize>()
            .expect("Parsing failed");

        // Push individual bets to the hash map
        bet_map.insert(hand.to_string(), bet);
    }

    // Define a function to sort poker hands
    fn custom_compare(a: &String, b: &String) -> std::cmp::Ordering {
        // Define a function that returns the hand type
        // Five of a kind - 7
        // Four of a kind - 6
        // Full house - 5
        // Three of a kind - 4
        // Two pair - 3
        // One pair - 2
        // High card - 1
        fn get_hand_type(hand: String) -> u32 {
            // Get all characters and unique characters as vectors
            let all_chars: Vec<char> = hand.as_str().chars().collect();
            let unique_chars: Vec<char> = hand.as_str().chars().unique().collect();

            // Five of a kind
            if unique_chars.len() == 1 {
                return 7;
            }

            // Four of a kind or Full house
            if unique_chars.len() == 2 {
                let count: usize = all_chars.iter().filter(|&&x| x == all_chars[0]).count();
                // Four of a kind
                if count == 1 || count == 4 {
                    return 6;
                }
                // Full house
                else {
                    return 5;
                }
            }

            // Three of a kind or Two pair
            if unique_chars.len() == 3 {
                // Get max count of first two chars - two should be enough to find a Two pair
                let count_1: usize = all_chars.iter().filter(|&&x| x == all_chars[0]).count();
                let count_2: usize = all_chars.iter().filter(|&&x| x == all_chars[1]).count();
                let max_count: usize = count_1.max(count_2);
                // Three of a kind
                if max_count != 2 {
                    return 4;
                }
                // Two pair
                else {
                    return 3;
                }
            }

            // One pair
            if unique_chars.len() == 4 {
                return 2;
            }
            // High card
            return 1;
        }

        // Return comparison of hand type for both hands
        return get_hand_type(a.to_string()).cmp(&get_hand_type(b.to_string()));
    }

    // Define a function to sort first high card in equal poker hands
    fn custom_compare_equal(a: &String, b: &String) -> std::cmp::Ordering {
        // Convert String to vector
        let all_chars_a: Vec<char> = a.as_str().chars().collect();
        let all_chars_b: Vec<char> = b.as_str().chars().collect();

        // Define a hash map with char to u32 mapping
        let mut char_map: HashMap<char, u32> = HashMap::new();
        char_map.insert('A', 1);
        char_map.insert('K', 2);
        char_map.insert('Q', 3);
        char_map.insert('J', 4);
        char_map.insert('T', 5);
        char_map.insert('9', 6);
        char_map.insert('8', 7);
        char_map.insert('7', 8);
        char_map.insert('6', 9);
        char_map.insert('5', 10);
        char_map.insert('4', 11);
        char_map.insert('3', 12);
        char_map.insert('2', 13);

        // Initialize an Ordering value
        let mut return_value: std::cmp::Ordering = std::cmp::Ordering::Equal;

        // Iterate over all five cards in a hand to resolve ties
        for i in 0..5 {
            if all_chars_a[i] != all_chars_b[i] {
                // Obtain card to rank mapping
                let map_a: u32 = *char_map.get(&all_chars_a[i]).expect("Retrieving failed");
                let map_b: u32 = *char_map.get(&all_chars_b[i]).expect("Retrieving failed");
                return_value = map_b.cmp(&map_a);
                // Stop once first unequal value has been compared
                break;
            }
        }
        return return_value;
    }

    // Sort by hand type followed by first high card
    hands.sort_by(|a, b| custom_compare(a, b).then_with(|| custom_compare_equal(a, b)));

    // Initialize variable to store rank*bet
    let mut sum: usize = 0;
    for i in 0..hands.len() {
        let entry: usize = (i + 1) * bet_map.get(&hands[i]).expect("Retrieving failed");
        sum += entry;
    }

    // Print final result
    println!("{:?}", sum);
}
