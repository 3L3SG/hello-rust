use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("\nPlease input your guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The secret number is: {}", &secret_number);
        println!("You guessed: {}", &guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry, too small!"),
            Ordering::Greater => println!("Sorry, too big!"),
            Ordering::Equal => {
                println!("Damn, you are right!");
                break;
            }
        };
    }
}
