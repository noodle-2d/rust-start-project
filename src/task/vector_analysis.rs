use crate::util::input;

pub fn run_task() {
    let vector = get_number_vector_from_stdin();
    print_vector(&vector, "Entered vector");

    let sorted_vector = sort_number_vector(vector);
    print_vector(&sorted_vector, "Sorted vector");
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

fn sort_number_vector(vector: Vec<i32>) -> Vec<i32> {
    // todo: implement sorting
    vector
}
