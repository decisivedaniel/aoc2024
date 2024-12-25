use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for row in contents.split('\n') {
        let (lstr, rstr) = row.split_once(' ')
        .expect("Expecting a space in the row");
        
        println!("left: {lstr}, right: {rstr}");
        left.push(lstr.trim().parse::<i32>().unwrap());
        right.push(rstr.trim().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    let mut count = 0;
    while count < left.len() {
        sum += (left[count] - right[count]).abs();
        count += 1;
    }
    
    println!("Sum of diff is: {sum}");
}
