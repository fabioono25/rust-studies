use std::io;

fn main() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    println!("Guess the number game {sparkle_heart}!");

    print!("Please input your guess: ");

    // creating a mutable variable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
