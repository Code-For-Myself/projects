use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        } 
        let query = args[1].clone();
        let filepath = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{ query, filepath, ignore_case})
    }
} 

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(()) 
}
pub fn search<'a>(
    query: &str,
    contents: &'a str,
    ) -> Vec<&'a str> {
        let mut result = Vec::new();
        for line in contents.lines(){
            if line.contains(&query){
                result.push(line);
            }
        }
        result
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_ascii_lowercase();
    let mut result = Vec::new();

        for line in content.lines(){
            if line.to_lowercase().contains(&query){
                result.push(line);
            }
        }
        result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents) 
        );
    }
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick Three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}