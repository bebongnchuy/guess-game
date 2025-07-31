use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::rng().random_range(1..=10);
    println!("Hello, world!");

    println!("correct: {correct}");
    println!("Hey, guess a number 1-10:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading guess");

    let guess: u32 = guess.trim().parse().expect("Error with parse."); // changing the datatype of guess to an unsigned integer
    // using if expressions
    /*if guess < correct {
        println!("You guessed too low, try again")
    } else if guess > correct {
        println!("You guessed too high, try again")
    } else {
        println!("You guessed the correct number")
    }*/

    // let message = if guess > correct {
    //     String::from("You guessed too high.")
    // } else if guess < correct {
    //     String::from("You guessed too low")
    // } else {
    //     String::from("You guessed CORRECT")
    // };

    // Another method
    let message = match guess.cmp(&correct) {
        Ordering::Greater => "You guessed too high",
        Ordering::Less => "You guessed too low",
        Ordering::Equal => "You guessed CORRECT",
    };

    println!("{message}\n\nYou guessed: {}", guess);
}
