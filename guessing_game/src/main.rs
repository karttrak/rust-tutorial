use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess as an integer between 1 and 100:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().parse::<i32>();
        if guess.is_err() {
            println!("Your guess is not a valid number. Try again.");
            continue;
        }

        let guess: i32 = guess.expect("Please enter a number");

        if guess == secret_number {
            println!("You win!");
            break;
        } else if guess < secret_number {
            println!("Too low. Try again.");
        } else if guess > secret_number {
            println!("Too high. Try again.");
        } else if guess != secret_number {
            println!("Something went wrong. Quit trying to break the game! Try again, following the rules this time.");
        }
    }
}
