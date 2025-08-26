use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
      println!("Guess the number");

      let mut rng = rand::rng();

      let secret_number:u32 = rng.random_range(0..=100);  

      //println!("The Secret Number is :{}",secret_number);

      loop{
            println!("Please input your guess: ");
      
            let mut guess =String::new();
            
            io::stdin().read_line(&mut guess).expect("Failed to read Line");

            let guess:u32 = match guess.trim().parse() {
                  Ok(num) => num,
                  Err(_) => continue
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number){
                  Ordering::Less => println!("Too Small!"),
                  Ordering::Greater => println!("Too Big!"),
                  Ordering::Equal => {
                              println!("You Win, Perfect Guess!");
                              break;
                        }                  
            }
      }
}
