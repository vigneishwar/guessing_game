use std::io;

fn main() {
    println!("Guess the num!");

    println!("Enter your guess: ");

    let mut guess = String::new(); // by default variables are immutable in rust. Use the keyword mut to make it mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("you guessed: {}", guess);
}
