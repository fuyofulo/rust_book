use std::io::stdin as yolo;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        
        println!("Guess a random number");
        println!("Enter the number you guessed: ");

        let mut guess = String::new();

        yolo()
            .read_line(&mut guess)
            .expect("Failed to read the number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //
}
