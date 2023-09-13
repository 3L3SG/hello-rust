use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the line");

    println!("The secret numbner is: {}", &secret_number);
    println!("You guessed: {}", &guess);
}
