
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Seed {secret_number}");
    
    
    println!("Hello, world2!");


    println!("Guess the Number!");
     loop {



        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You Guessed: {guess}");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {println!("You win!"); break;}

        }
    }
}
