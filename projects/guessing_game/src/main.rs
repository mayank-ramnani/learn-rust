use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, This is a number guessing game!");
    // generate a secret number
    let secret = rand::thread_rng().gen_range(1..100);
    // println!("generated: {}", secret);
    loop {
        println!("Guess a number between 1 to 100: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error in parsing: {}", guess);
                continue;
            }
        };
        println!("current guess: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
