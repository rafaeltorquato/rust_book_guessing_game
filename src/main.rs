use std::cmp::Ordering;
use std::io;

use colored::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries_count: u32 = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number!");
                continue;
            }
        };

        println!("You guessed: {}.", guess);
        tries_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                let msg = format!("You win with {} tries!", tries_count).green();
                println!("{}", msg);
                break;
            }
        };
    }
}
