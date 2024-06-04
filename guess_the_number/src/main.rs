use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess a number between 1 and 100! U have 7 tries!");
    let mut counter = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    loop {
        counter += 1;
        if counter == 8{
            println!("You lose!");
            break;
        }
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };

        println!("U guessed: {}", guess); 

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
