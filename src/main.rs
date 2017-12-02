use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a path to the datafile");
        return;
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => (),//print!("{} contains:\n{}", display, s),
    }

    day1_problem1(s.as_str());
}

fn day1_problem1(data: &str) {
    let mut sum: u32 = 0;
    
    for (i, c) in data.chars().enumerate() {
        let mut curr: u32 = c.to_digit(10).unwrap();
        let mut next: u32 = 0;

        if i + 1 == data.len() {
            next = data.chars().nth(0).unwrap().to_digit(10).unwrap();
        } else {
            next = data.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
        }

        if curr == next {
            sum += curr;
        }
        // do something with character `c` and index `i`
    }

    println!("Result = {}", sum);
}