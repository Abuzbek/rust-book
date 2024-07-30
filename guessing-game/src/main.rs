use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_key = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_key);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_key) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "Too win!".green());
                break;
            }
        }
    }
}
