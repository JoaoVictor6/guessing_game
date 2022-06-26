use std::{io, cmp::Ordering};
use rand::prelude::*;

// range expression
// gen.range(1..100) = 1,2,3,4,5...99
// geb.range(1..=100) = 1,2,3,4,5...100 
fn main() {
  let random_number = rand::thread_rng().gen_range(1..=100);
  
  loop {
    println!("----Guess the number!");
    println!("> Please input yout guess.");
    let mut guess = String::new(); 
    
    io::stdin()
      .read_line(&mut guess)
      .expect(">Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed: {guess}");
    //or println!("You guessed: {}", guess);
    
    match guess.cmp(&random_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win");
        break;
      }
    }
  }
  println!("Secret number is {random_number}");
}
