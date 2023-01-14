use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Failed to parse input");

    pyramid_function(number);
}

fn pyramid_function(x: i32){
    for row in 0..x+1{
        for col in 0..row{
            print!("* ");
        }
        println!("");
    }
}