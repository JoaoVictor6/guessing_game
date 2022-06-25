use std::io;
use rand::prelude::*;

// range expression
// gen.range(1..100) = 1,2,3,4,5...99
// geb.range(1..=100) = 1,2,3,4,5...100 
fn main() {
  println!("----Guess the number!");
  println!("> Please input yout guess.");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect(">Failed to read line");
  let random_number = rand::thread_rng().gen_range(1..=100);
  println!("You guessed: {guess}\nResult: {}", random_number);
  //or println!("You guessed: {}", guess);
}
