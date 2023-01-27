use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(&config.file_path).expect("file should exist");

    println!("searching for {}", config.querry);
    println!("in file: {}", config.file_path);
    println!("file contents:\n{}", contents);
}

struct Config {
    querry: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Requires a querry and a file path");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { querry, file_path })
    }
}
