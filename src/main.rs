use rand::Rng;
use std::io;

fn main() {
    println!("Guess the num!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("Enter your sec num {}", secret_num);

    println!("Enter your guess: ");

    let mut guess = String::new(); // by default variables are immutable in rust. Use the keyword mut to make it mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("you guessed: {}", guess);
}
