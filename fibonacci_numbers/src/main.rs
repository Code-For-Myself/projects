use std::io;
fn main() {
    println!("Which Fibonacci Number (index) u want to know?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let n: u64 = input.trim().parse().expect("Please enter a number");
    println!("The Fibonacci number with index {n} is {}", fibonacci(n));
}
fn fibonacci(n: u64) -> u64 { 
    let mut counter = 0;
    let mut fnumber = 0;
    let mut x = 0;
    let mut y =1;
    if counter == n {
        return fnumber;
    } else if n == 1 {
        fnumber = 1;
        return fnumber;
    }else {
        counter = 2;
        loop{
            x = x + y;
            counter += 1;
            if counter == n {
                return x;
            }
            y = x + y;
            counter += 1;
            if counter == n {
                return y;
            }

        }
    }
}
