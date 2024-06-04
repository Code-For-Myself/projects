use std::io;

const FACTOR: f32 = 1.8;
const FACTOR_TWO: f32 = 32.0;

fn main() {
     let mut input = String::new();
     loop {
          println!("Choose if u want to calculate celsius to fahrenheit or the other way around");
          println!("(1) Celsius to Fahrenheit");
          println!("(2) Fahrenheit to Celsius");
          io::stdin()
               .read_line(&mut input)
               .expect("Failed to read line");
          let input: u32 = input.trim().parse().expect("Please type a number");
          match &input {
               1 => {
                    println!("Enter a temperature in Celsius!");
                    break;
               },
               2 => {
                    println!("Enter a temperature in Fahrenheit!");
                    break;
                    
               },
               _ => {
                    println!("Invalid input. Enter 1 or 2!"); 
               }
               
          };
     }
     let mut amount = String::new();
     io::stdin()
               .read_line(&mut amount)
               .expect("Failed to read line");
     let amount: f32 = amount.trim().parse().expect("Please type a number");
     let input: u32 = input.trim().parse().expect("Please type a number");
     match input {
          1 => {
               let result = amount * FACTOR + FACTOR_TWO;
               println!("{amount}°C equals {result} Grad Fahrenheit.");
          },
          2 => {
               let result = (amount -FACTOR_TWO)/FACTOR;
               println!("{amount} Grad Fahrenheit equals {result}°C.");
          },
          _ => {
               print!("invalid");
          }
     }

   
}

