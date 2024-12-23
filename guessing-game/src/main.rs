use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Let's play a game, guess the number!");
    
    loop {   
        println!("Take a guess: ");

        let mut guess = String::new();     
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // println!("The answer was {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low! 🤏"),
            Ordering::Greater => println!("Your guess is too high! 🌿🚬🌬️"),
            Ordering::Equal => {
                println!("Your guess is correct! You win 🤯!");
                break;
            }
        }
    }

}
