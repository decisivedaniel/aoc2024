use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file")
    .split_whitespace();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    

    left.sort();
    right.sort();
    println!("Hello, world!");
}
