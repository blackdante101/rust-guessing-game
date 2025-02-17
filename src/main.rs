use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Welcome to the guessing game!");

    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("secret number: {}", secret_number);
    loop{
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line");

        let guess:u32 = guess.trim().parse().expect("Please enter a number");

        println!("you guessed: {}",guess);

        match guess.cmp(&secret_number){ 
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Hurray! You've won");
                break;
            }
        }
    }
    
}
