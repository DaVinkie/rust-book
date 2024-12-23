use std::io;

fn main() {
    println!("Let's play a game, guess the number!");
    println!("Take a guess: ");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");

    println!("You guessed: {}", guess);
}
