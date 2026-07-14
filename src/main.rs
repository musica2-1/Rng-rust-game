use std::cmp::Ordering;   
use std::io;   

use rand::RngExt;

fn main() {
    println!("Guess the numbaa!!!");

    let secret_number = rand::rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)   //-Le apenas input string
            .expect("deu erro bro");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessou: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
