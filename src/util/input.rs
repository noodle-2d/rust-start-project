use rand::Rng;
use std::io;
use std::num::ParseIntError;

pub fn get_random_number() -> u32 {
    rand::thread_rng().gen_range(1..101)
}

pub fn get_string_from_stdin() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}

pub fn get_number_from_stdin() -> Result<u32, ParseIntError> {
    get_string_from_stdin().trim().parse()
}
