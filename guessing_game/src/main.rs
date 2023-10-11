use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    println!("Guess the number game {sparkle_heart}!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=101);
    println!("The secret number is: {secret_number}");

    loop {
        print!("Please input your guess: ");

        // creating a mutable variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: i32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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
