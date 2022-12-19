use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the num!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("Enter your sec num {}", secret_num);

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new(); // by default variables are immutable in rust. Use the keyword mut to make it mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = guess.trim().parse().expect("Please type a number"); // shadowing

        println!("you guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You win ");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
