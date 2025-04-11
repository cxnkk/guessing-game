use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let correct = rand::rng().random_range(1..=100);
    println!("Correct: {correct}");
    println!("Hey, guess a number 1-100:");
    
    loop {
     let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Error reading input");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(e) => {
            println!("Error with parse, try again. {e}");
            continue;
        }
    };
    
    match guess.cmp(&correct){
        Ordering::Greater => println!("WRONG! You guessed to high."),
        Ordering::Less => println!("WRONG! You guessed to low."),
        Ordering::Equal => {println!("CONGRATS! You guessed correct."); break;},
    };    
    }
}