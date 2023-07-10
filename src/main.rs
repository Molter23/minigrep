use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[0].clone();
        let file_path = args[1].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args);

    let content =
        fs::read_to_string(config.file_path).expect("Should have been able to open the file");

    println!("Content {}", content);
}
