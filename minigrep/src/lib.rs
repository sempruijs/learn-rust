use std::error::Error;
use std::fs;

pub struct Config {
    pub querry: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Requires a querry and a file path");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { querry, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("file contents:\n{}", contents);

    Ok(())
}
