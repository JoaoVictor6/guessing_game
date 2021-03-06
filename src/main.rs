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
    
    let guess: u32 = match guess.trim().parse() { // parser func returns a enum with two values: Ok or Err, using match for handling error
      Ok(num) => num,
      Err(_) => {
        let guess = guess.trim();
        println!("{guess} is not be converted to number!");
        continue;
      },
    };
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
