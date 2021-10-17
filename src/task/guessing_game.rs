use std::cmp::Ordering;
use crate::util::input;

pub fn run_task() {
    println!("Guess the number!");
    let secret_number = input::get_random_number();

    loop {
        println!("Please input your guess.");
        let guess = match input::get_number_from_stdin() {
            Ok(number) => number,
            Err(_) => {
                println!("You entered a string, that is not a number. Please try again.");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
