use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

enum Error {
    NoArgs,
    QueryError,
    FilePathError,
    ToMuchArgs
}

impl Config {
    fn new(args: &[String]) -> Result<Config, Error> {
        match args.len() {
            1 => Err(Error::NoArgs),
            2 => Err(Error::QueryError),
            3 => {
                let query = args[1].clone();
                let file_path = args[2].clone();
        
                Ok(Config { query, file_path })
            }
            _ => Err(Error::ToMuchArgs),
        
        }
       
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(config) = Config::new(&args) {
        let content =
        fs::read_to_string(config.file_path).expect("Should have been able to open the file");

    println!("Content {}", content);
    } else {
        panic!("error");
    }


}
