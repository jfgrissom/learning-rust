use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a secret to be compared to the user's guess
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable variable to store the user's guess.
        // Note here guess is declared as a mutable string variable.
        let mut guess = String::new();

        // Read the user's guess from the console
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Implement a crude kind of validation to ensure the user's guess is a number
        // Note here guess is shadowed, immutable, and declared as a u32 variable.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        //.expect("Please type a number!");

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
