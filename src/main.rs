use std::io;
fn main() {
    // Given a set of integers, Return the measure of central tendency: Mean, Median, Mode
    let mut input_values = Vec::new();

    println!("Enter input values (one per line). Enter 'done' to finish:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        if input == "done" {
            break;
        }

        let input_value: i32 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input, please enter a valid integer");
                continue;
            }
        };

        input_values.push(input_value);
    }

    println!("Input vector: {:?}", input_values);
    // Find the mean value of the input_values vector
    fn find_mean(values: &Vec<i32>) -> Option<f32> {
        if values.is_empty() {
            // Handle the case where the vector is empty
            println!("Enter values for the data set");
            None
        } else {
            let mut sum: f32 = 0.0;
            for value in values {
                sum += *value as f32;
            }
            Some(sum / values.len() as f32)
        }
    }

    let mean = match find_mean(&mut input_values) {
        Some(mean) => mean,
         _ => 0.0
    };
    println!("Mean: {:.3}", mean);
    // Find the median value of the input_values vector
    fn find_median(mut values: Vec<i32>) -> Option<i32> {
        if values.is_empty() {
            println!("Enter values for the data set");
            None
        } else {
            values.sort();
            Some(values[values.len() / 2])
        }
    }
    let median = match find_median(input_values) {
        Some(median) => median,
        _ => 0
    };
    println!("Median: {}", median);
}
