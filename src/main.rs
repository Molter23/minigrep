use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let query = &args[0];
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
                    .expect("Should have been able to open the file");

    println!("Content {}", content);
}
