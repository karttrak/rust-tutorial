use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess as an integer between 1 and 100:");

    let mut guess_int: i32 = -1;

    while guess_int != secret_number {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let result = guess.trim().parse::<i32>();
        if result.is_err() {
            println!("Your guess is not a valid number. Try again.");
            continue;
        }

        let guess: i32 = result.expect("Please enter a number");

        guess_int = guess;

        if guess_int < secret_number {
            println!("Too low. Try again.");
        } else if guess_int > secret_number {
            println!("Too high. Try again.");
        } else if guess_int != secret_number {
            println!("Something went wrong. Quit trying to break the game! Try again, following the rules this time.");
        }
    }

    println!("You win!");
}
