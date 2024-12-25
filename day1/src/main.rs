use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for row in contents.split('\n') {
        let (lstr, rstr) = row.split_once(' ')
        .expect("Expecting a space in the row");
        
        println!("left: {lstr}, right: {rstr}");
        left.push(lstr.trim().parse::<i32>().unwrap());
        let count = right.entry(rstr.trim().parse::<i32>().unwrap()).or_insert(0);
        *count += 1;
    }

    left.sort();

    let mut sum = 0;
    let mut count = 0;
    while count < left.len() {
        sum += (left[count] * right.get(&left[count]).unwrap_or(0_)).abs();
        count += 1;
    }
    
    println!("Sum of diff is: {sum}");
}
