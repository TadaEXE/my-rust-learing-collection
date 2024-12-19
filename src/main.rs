use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Number guessing game");

    let number = rand::thread_rng().gen_range(1..=69);

    loop {
        println!("Input a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        match guess.as_str().trim() {
            "cheat" => println!("Number is :{number}"),
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
