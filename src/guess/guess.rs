use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");
    println!("Pleas input your number!");
    let target: u32 = rand::thread_rng().gen_range(0..101);

    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Can't not convert the guess, please retry:");
                continue;
            }
        };
        match guess.cmp(&target) {
            Ordering::Greater => println!("{}", "Too big!".red()), 
            Ordering::Equal => {
                println!("{}", "Pingo!".green()); 
                break
            },
            Ordering::Less => println!("{}", "Too small!".yellow()),
        }
    }
}