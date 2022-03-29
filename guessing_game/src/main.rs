use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

const MIN_NUM: u32 = 1;
const MAX_NUM: u32 = 100;

fn main() {
    // println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(MIN_NUM..=MAX_NUM);

    // println!("The secret number is: {}", secret_number);

    let mut curr_min = MIN_NUM;
    let mut curr_max = MAX_NUM;

    let mut num_tries: u32 = 0;

    loop {
        print!(
            "Please input a guess between {} and {}, or type quit to exit game: ",
            curr_min, curr_max
        );
        std::io::stdout().flush().expect("Could not flush.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq("quit") {
            println!("Game terminated.");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number, or type quit to exit game.");
                println!();
                continue;
            }
        };

        // println!("You guessed: {}", guess);
        num_tries += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                if guess > curr_min {
                    curr_min = guess + 1;
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                if guess < curr_max {
                    curr_max = guess - 1;
                }
            }
            Ordering::Equal => {
                println!("You win in {} tries!", num_tries);
                break;
            }
        }
        println!();
    }
}
