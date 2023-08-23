use std::{collections::HashMap, io};
fn main() {
    // Create an empty vector to store the input values
    let mut input_values = Vec::new();

    // Prompt the user to enter input values, one per line
    println!("Enter input values (one per line). Enter 'done' to finish:");

    // Start an infinite loop to continuously read user input until 'done' is entered
    loop {
        // Create a mutable string to store the user input
        let mut input = String::new();

        // Read a line of input from the user and store it in the 'input' variable
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Remove any leading or trailing whitespace from the input
        input = input.trim().to_string();

        // If the user entered 'done', exit the loop
        if input == "done" {
            break;
        }

        // Attempt to parse the input string into an integer
        let input_value: i32 = match input.parse() {
            // If parsing succeeds, store the parsed value in 'input_value'
            Ok(value) => value,
            // If parsing fails, print an error message and continue to the next iteration of the loop
            Err(_) => {
                println!("Invalid input, please enter a valid integer");
                continue;
            }
        };

        // Add the parsed input value to the 'input_values' vector
        input_values.push(input_value);
    }
    // Print the input vector
    println!("Input vector: {:?}", input_values);

    // Call the find_mean function with a mutable reference to the input_values vector
    let mean = match find_mean(&mut input_values) {
        // If the mean is Some(mean), assign it to the mean variable
        Some(mean) => mean,
        // If the mean is None, assign 0.0 to the mean variable
        _ => 0.0,
    };

    // Print the mean value
    println!("Mean: {:.3}", mean);

    // Call the find_median function with a mutable reference to the input_values vector
    let median = match find_median(&mut input_values) {
        // If the median is Some(median), assign it to the median variable
        Some(median) => median,
        // If the median is None, assign 0 to the median variable
        _ => 0,
    };

    // Print the median value
    println!("Median: {}", median);

    // Call the 'find_mode' function with a mutable reference to the 'input_values' variable
    let modes = find_mode(&mut input_values);

    // Check if the 'modes' vector is empty
    if modes.is_empty() {
        // Print a message indicating that no mode was found
        println!("No mode found");
    } else {
        // Print the modes vector using debug formatting
        println!("Modes: {:?}", modes);
    }
}

// Find the median value of the input_values vector
fn find_median(values: &mut Vec<i32>) -> Option<i32> {
    // Check if the vector is empty
    if values.is_empty() {
        println!("Enter values for the data set");
        None
    } else {
        // Sort the vector in ascending order
        values.sort();
        // Calculate the median by taking the middle value of the sorted vector
        Some(values[values.len() / 2])
    }
}

fn find_mode(values: &mut Vec<i32>) -> Vec<i32> {
    // Check if the values vector is empty
    if values.is_empty() {
        println!("Enter values for the data set");
        return vec![]; // Return an empty vector if there are no values
    }

    // Sort the values vector in ascending order
    values.sort();

    // Create a HashMap to store the frequency count of each value
    let mut map = HashMap::new();

    // Initialize a variable to keep track of the maximum count
    let mut max_count = 0;

    // Iterate over each value in the values vector
    for value in values {
        // Update the count for the current value in the HashMap
        // If the value is not present in the HashMap, insert it with a count of 0
        let count = map.entry(value).or_insert(0);
        *count += 1; // Increment the count for the current value

        // Update the max_count if the count for the current value is greater
        if *count > max_count {
            max_count = *count;
        }
    }

    // Declare a mutable vector to store the modes
    let mut modes = vec![];

    // Iterate over the key-value pairs in the 'map' variable
    for (key, value) in &map {
        // Check if the value is equal to the maximum count
        if value == &max_count {
            // Append the key to the 'modes' vector
            modes.push(**key);
        }
    }

    // Return the 'modes' vector
    modes
}
// Find the mean value of the input_values vector
fn find_mean(values: &Vec<i32>) -> Option<f32> {
    // Check if the vector is empty
    if values.is_empty() {
        // Handle the case where the vector is empty
        println!("Enter values for the data set");
        None
    } else {
        let mut sum: f32 = 0.0;
        // Iterate over each value in the vector
        for value in values {
            // Add the value to the sum
            sum += *value as f32;
        }
        // Calculate the mean by dividing the sum by the length of the vector
        Some(sum / values.len() as f32)
    }
}
