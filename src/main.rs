use std::io;
use rand::Rng;
fn main() {
      println!("Guess the number");

      let rng = rand::rng().random_range(0..=100);  

      println!("The Secret Number is :{}",rng);

      println!("Please input your guess");
      
      let mut guess =String::new();
      
      io::stdin().read_line(&mut guess).expect("Failed to read Line");

      print!("You guessed :{}",guess);
}
