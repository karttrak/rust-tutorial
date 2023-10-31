use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100! Please enter your guess as an integer.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess is not a valid number. Try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low. Try again."),
            Ordering::Greater => println!("Too high. Try again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
