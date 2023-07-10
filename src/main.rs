use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let query = &args[0];
    let file_path = &args[1];

    println!("{} {}", query, file_path);
}
