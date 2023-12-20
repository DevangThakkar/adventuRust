// Import necessary modules and libraries
use std::collections::HashMap;
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

    // Initialize hashmap that stores box label: [[label1, focal_len1], [label2, focal_len2]]
    let mut box_map: HashMap<usize, Vec<Vec<String>>> = HashMap::new();

    // Iterate over each step and obtain its corresponding hash
    for step in steps {
        // Initialize a variable to hold value for each step - this is the box label
        let mut box_label: usize = 0;

        // Obtain the label from each step
        let label: &str = if step.chars().collect::<Vec<char>>().contains(&'-') {
            step.split("-").collect::<Vec<&str>>()[0]
        } else {
            step.split("=").collect::<Vec<&str>>()[0]
        };

        // Obtain operation
        let operation: char = if step.chars().collect::<Vec<char>>().contains(&'-') {
            '-'
        } else {
            '='
        };

        // Obtain focal length
        let focal_len: usize = if step.chars().collect::<Vec<char>>().contains(&'-') {
            0
        } else {
            step.split("=").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .expect("Parsing failed")
        };

        // Use the hash technique to obtain box label for each step
        for i in 0..label.len() {
            let character: char = step.chars().nth(i).expect("Retrieving failed");
            let ascii: usize = character as usize;
            box_label += ascii;
            box_label *= 17;
            box_label %= 256;
        }

        // If operation adds, add if not present, else replace
        if operation == '=' {
            // If box is not empty
            if box_map.contains_key(&box_label) {
                // Find where the box label is present in the vector
                let (mut present, mut counter): (usize, usize) = (0, 0);

                // Iterate over all values for a box to see if label is present and store index
                // Could be done smarter I guess
                for i in 0..box_map[&box_label].len() {
                    let item = &box_map[&box_label][i];
                    if item[0] == label.to_string() {
                        present = 1;
                        counter = i;
                    }
                }

                let mut current_val = box_map[&box_label].clone();

                // If label is not present in the box, add it at the end
                if present == 0 {
                    current_val.push(vec![label.to_string(), focal_len.to_string()]);
                    box_map.insert(box_label, current_val.clone());
                }
                // If label is present in the box, update focal length
                else {
                    current_val[counter][1] = focal_len.to_string();
                    box_map.insert(box_label, current_val.clone());
                }
            }
            // If box is empty
            else {
                box_map.insert(
                    box_label,
                    vec![vec![label.to_string(), focal_len.to_string()]],
                );
            }
        }
        // If the operation removes, you can only remove if present
        else {
            // If box is not empty
            if box_map.contains_key(&box_label) {
                // Find where the box label is present in the vector
                let (mut present, mut counter): (usize, usize) = (0, 0);

                // Iterate over all values for a box to see if label is present and store index
                // Could be done smarter I guess
                for i in 0..box_map[&box_label].len() {
                    let item = &box_map[&box_label][i];
                    if item[0] == label.to_string() {
                        present = 1;
                        counter = i;
                    }
                }

                let mut current_val = box_map[&box_label].clone();

                // Remove if present
                if present == 1 {
                    current_val.remove(counter);
                    box_map.insert(box_label, current_val.clone());
                }
            }
        }
    }

    // Calculate score
    for (key, value) in box_map.iter() {
        if value.len() != 0 {
            for i in 0..value.len() {
                let item = &value[i];
                sum += (key + 1) * (i + 1) * (item[1].parse::<usize>().expect("Parsing failed"));
            }
        }
    }

    println!("{:?}", sum);
}
