use std::io;
use rand::Rng;
use std::cmp::Ordering;
use term_painter::Color::*;

use term_painter::ToStyle;
fn main() {
    println!("{}", Red.paint("Guess the secret number!"));
    println!("{}",  Red.paint("Please input your guess."));
    
    let secret_number = rand::thread_rng().gen_range(1..101);
   

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}{}", Green.paint("You guessed:"), Green.paint(guess));
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", Magenta.paint("Too small!")),
            Ordering::Greater => println!("{}" , BrightYellow.paint("Too big!")),
            Ordering::Equal =>{
                println!("{} {}", BrightCyan.paint("You win! the secret number was:"), secret_number);
                break;
            }
        }


    }
}
