use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];

    println!("Query is {}", query);
    println!("Filepath is {}", filepath);

    let content = fs.read_to_string(filepath)
                    .expect("Should've been able to read filepath");

    println!("With text:\n{contents}");

}