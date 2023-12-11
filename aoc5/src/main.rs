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

    // Define the section that is currently being parsed
    let mut section: &str = "seeds";

    // Initialize a vector to store different steps
    let mut seeds: Vec<u64> = Vec::new();
    let mut soils: Vec<u64> = Vec::new();
    let mut fertilizers: Vec<u64> = Vec::new();
    let mut waters: Vec<u64> = Vec::new();
    let mut lights: Vec<u64> = Vec::new();
    let mut temperatures: Vec<u64> = Vec::new();
    let mut humidities: Vec<u64> = Vec::new();
    let mut locations: Vec<u64> = Vec::new();

    // Initialize a vector to store the mapping between different steps
    let mut seed_to_soil_vec: Vec<Vec<u64>> = Vec::new();
    let mut soil_to_fertilizer_vec: Vec<Vec<u64>> = Vec::new();
    let mut fertilizer_to_water_vec: Vec<Vec<u64>> = Vec::new();
    let mut water_to_light_vec: Vec<Vec<u64>> = Vec::new();
    let mut light_to_temperature_vec: Vec<Vec<u64>> = Vec::new();
    let mut temperature_to_humidity_vec: Vec<Vec<u64>> = Vec::new();
    let mut humidity_to_location_vec: Vec<Vec<u64>> = Vec::new();

    // Read lines from the specified file
    let lines = read_lines(file_path).expect("Reading failed");

    // Iterate over each line in the file
    for unchecked_line in lines {
        // Unwrap the line or panic if unwrapping fails
        let line: String = unchecked_line.expect("Iterating failed");

        // Split over colon to get seeds or section type
        let line_split: Vec<&str> = line.split(":").collect();

        // Define the section that is currently being parsed
        let mut data_type: &str = "";

        if line_split.len() > 1 {
            data_type = line_split.get(0).expect("Parsing failed");
        }
        if data_type == "seed-to-soil map" {
            section = "seed-to-soil map";
        }
        if data_type == "soil-to-fertilizer map" {
            section = "soil-to-fertilizer map";
        }
        if data_type == "fertilizer-to-water map" {
            section = "fertilizer-to-water map";
        }
        if data_type == "water-to-light map" {
            section = "water-to-light map";
        }
        if data_type == "light-to-temperature map" {
            section = "light-to-temperature map";
        }
        if data_type == "temperature-to-humidity map" {
            section = "temperature-to-humidity map";
        }
        if data_type == "humidity-to-location map" {
            section = "humidity-to-location map";
        }

        // Push maps of the form [source_start, source_end, destination_start]
        match section {
            "seeds" => {
                if line.starts_with("seeds") {
                    // Parse seed data and store in the seeds vector
                    seeds = line_split
                        .get(1)
                        .expect("Parsing failed")
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                }
            }
            "seed-to-soil map" => {
                // Parse seed-to-soil map data and store in the seed_to_soil_vec vector
                if line == "" || line == "seed-to-soil map:" {
                    continue;
                } else {
                    let mut seed_to_soil: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = seed_to_soil[0];
                    seed_to_soil[0] = seed_to_soil[1];
                    seed_to_soil[1] = seed_to_soil[1] + seed_to_soil[2];
                    seed_to_soil[2] = first;
                    seed_to_soil_vec.push(seed_to_soil);
                }
            }
            "soil-to-fertilizer map" => {
                // Parse soil-to-fertilizer map data and store in the soil_to_fertilizer_vec vector
                if line == "" || line == "soil-to-fertilizer map:" {
                    continue;
                } else {
                    let mut soil_to_fertilizer: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = soil_to_fertilizer[0];
                    soil_to_fertilizer[0] = soil_to_fertilizer[1];
                    soil_to_fertilizer[1] = soil_to_fertilizer[1] + soil_to_fertilizer[2];
                    soil_to_fertilizer[2] = first;
                    soil_to_fertilizer_vec.push(soil_to_fertilizer);
                }
            }
            "fertilizer-to-water map" => {
                // Parse fertilizer-to-water map data and store in the fertilizer_to_water_vec vector
                if line == "" || line == "fertilizer-to-water map:" {
                    continue;
                } else {
                    let mut fertilizer_to_water: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = fertilizer_to_water[0];
                    fertilizer_to_water[0] = fertilizer_to_water[1];
                    fertilizer_to_water[1] = fertilizer_to_water[1] + fertilizer_to_water[2];
                    fertilizer_to_water[2] = first;
                    fertilizer_to_water_vec.push(fertilizer_to_water);
                }
            }
            "water-to-light map" => {
                // Parse water-to-light map data and store in the water_to_light_vec vector
                if line == "" || line == "water-to-light map:" {
                    continue;
                } else {
                    let mut water_to_light: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = water_to_light[0];
                    water_to_light[0] = water_to_light[1];
                    water_to_light[1] = water_to_light[1] + water_to_light[2];
                    water_to_light[2] = first;
                    water_to_light_vec.push(water_to_light);
                }
            }
            "light-to-temperature map" => {
                // Parse light-to-temperature map data and store in the light_to_temperature_vec vector
                if line == "" || line == "light-to-temperature map:" {
                    continue;
                } else {
                    let mut light_to_temperature: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = light_to_temperature[0];
                    light_to_temperature[0] = light_to_temperature[1];
                    light_to_temperature[1] = light_to_temperature[1] + light_to_temperature[2];
                    light_to_temperature[2] = first;
                    light_to_temperature_vec.push(light_to_temperature);
                }
            }
            "temperature-to-humidity map" => {
                if line == "" || line == "temperature-to-humidity map:" {
                    continue;
                } else {
                    let mut temperature_to_humidity: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = temperature_to_humidity[0];
                    temperature_to_humidity[0] = temperature_to_humidity[1];
                    temperature_to_humidity[1] =
                        temperature_to_humidity[1] + temperature_to_humidity[2];
                    temperature_to_humidity[2] = first;
                    temperature_to_humidity_vec.push(temperature_to_humidity);
                }
            }
            "humidity-to-location map" => {
                // Parse humidity-to-location map data and store in the humidity_to_location_vec vector
                if line == "" || line == "humidity-to-location map:" {
                    continue;
                } else {
                    let mut humidity_to_location: Vec<u64> = line
                        .trim()
                        .split(" ")
                        .map(|c| c.parse::<u64>().ok().unwrap())
                        .collect();
                    let first = humidity_to_location[0];
                    humidity_to_location[0] = humidity_to_location[1];
                    humidity_to_location[1] = humidity_to_location[1] + humidity_to_location[2];
                    humidity_to_location[2] = first;
                    humidity_to_location_vec.push(humidity_to_location);
                }
            }
            // Final case that should not occur with clean data
            &_ => {
                println!("Should not enter this");
            }
        }
    }

    // Process data vectors to compute the final location
    for seed in seeds.iter() {
        let soil = seed_to_soil_vec
            .iter()
            .find(|&v| v[0] <= *seed && *seed < v[1])
            .map(|v| &v[2] + seed - &v[0])
            .unwrap_or(*seed);
        soils.push(soil);
    }
    for soil in soils.iter() {
        let fertilizer = soil_to_fertilizer_vec
            .iter()
            .find(|&v| v[0] <= *soil && *soil < v[1])
            .map(|v| &v[2] + soil - &v[0])
            .unwrap_or(*soil);
        fertilizers.push(fertilizer);
    }
    for fertilizer in fertilizers.iter() {
        let water = fertilizer_to_water_vec
            .iter()
            .find(|&v| v[0] <= *fertilizer && *fertilizer < v[1])
            .map(|v| &v[2] + fertilizer - &v[0])
            .unwrap_or(*fertilizer);
        waters.push(water);
    }
    for water in waters.iter() {
        let light = water_to_light_vec
            .iter()
            .find(|&v| v[0] <= *water && *water < v[1])
            .map(|v| &v[2] + water - &v[0])
            .unwrap_or(*water);
        lights.push(light);
    }
    for light in lights.iter() {
        let temperature = light_to_temperature_vec
            .iter()
            .find(|&v| v[0] <= *light && *light < v[1])
            .map(|v| &v[2] + light - &v[0])
            .unwrap_or(*light);
        temperatures.push(temperature);
    }
    for temperature in temperatures.iter() {
        let humidity = temperature_to_humidity_vec
            .iter()
            .find(|&v| v[0] <= *temperature && *temperature < v[1])
            .map(|v| &v[2] + temperature - &v[0])
            .unwrap_or(*temperature);
        humidities.push(humidity);
    }
    for humidity in humidities.iter() {
        let location = humidity_to_location_vec
            .iter()
            .find(|&v| v[0] <= *humidity && *humidity < v[1])
            .map(|v| &v[2] + humidity - &v[0])
            .unwrap_or(*humidity);
        locations.push(location);
    }

    // Find the minimum location and print the result
    let min_location = locations.iter().cloned().min().expect("Minimizing failed");
    println!("{:?}", min_location);
}
