#![allow(dead_code)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess{
    value: i32, 
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 {
            panic!("Error value must be greater than 1, got {}", value);
        } else if value > 100 {
            panic!("Error value must be lesser than 100, got {}", value);
        }

        Guess {value: value}
    }
}

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("The secret number is: {}", secret_number);

    loop{

        println!("Please enter your guess :");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init_guess(){
        let x = 46;
        let guess = Guess::new(x);
        assert_eq!(guess.value, 46);
    }

    #[test]
    fn test_init_guess2(){
        Guess::new(43);
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
    Guess::new(433);
    }

    #[test]
    #[should_panic(expected= "Error value must be lesser than 100")]
    fn greater_than_100_msg(){
    Guess::new(433);
    }

    #[test]
    #[should_panic(expected= "Error value must be greater than 1")]
    fn lesser_than_1_msg(){
    Guess::new(-3);
    }
}