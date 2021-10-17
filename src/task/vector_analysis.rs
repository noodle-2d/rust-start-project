use std::collections::HashMap;
use crate::util::input;
use crate::util::sort;

pub fn run_task() {
    let vector = get_number_vector_from_stdin();
    print_vector(&vector, "Entered vector");

    let sorted_vector = sort::sort(vector);
    print_vector(&sorted_vector, "Sorted vector");

    if sorted_vector.is_empty() {
        println!("Vector is empty, it's impossible to find mean, median and mode")
    } else {
        let mean = find_mean(&sorted_vector);
        let median = find_median(&sorted_vector);
        let mode = find_mode(&sorted_vector);
        println!("Data for the given vector: mean = {}, median = {}, mode = {}", mean, median, mode);
    }
}

fn print_vector(vector: &Vec<i32>, description: &str) {
    println!("{}:", description);
    for (index, &number) in vector.iter().enumerate() {
        println!("vector[{}] = {}", index, number);
    }
    println!();
}

fn get_number_vector_from_stdin() -> Vec<i32> {
    println!("Print several integer numbers:");
    let string = input::get_string_from_stdin();

    let mut number_vector = Vec::new();
    for word in string.split_whitespace() {
        let number = word.parse().expect("Not a number!");
        number_vector.push(number);
    }

    number_vector
}

fn find_mean(vector: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for &item in vector.iter() {
        sum += item;
    }
    (sum as f64) / (vector.len() as f64)
}

fn find_median(vector: &Vec<i32>) -> f64 {
    let vector_length = vector.len();
    if vector_length % 2 == 0 {
        ((vector[vector_length / 2] + vector[vector_length / 2 - 1]) as f64) / 2.0
    } else {
        vector[vector_length / 2] as f64
    }
}

fn find_mode(vector: &Vec<i32>) -> i32 {
    let mut frequencies = HashMap::new();

    for &item in vector.iter() {
        let current_frequency_opt = frequencies
            .get(&item)
            .map(|i| { *i });
        match current_frequency_opt {
            Some(frequency) => frequencies.insert(item, frequency + 1),
            None => frequencies.insert(item, 0),
        };
    }

    let mut max_frequency = 0;
    let mut number_with_max_frequency = 0;

    for (&item, &frequency) in frequencies.iter() {
        if frequency > max_frequency {
            max_frequency = frequency;
            number_with_max_frequency = item;
        }
    }

    number_with_max_frequency
}
