use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let number:i32=input.trim().parse().expect("Please Type A Number!!!");
    
    
    for i in 1..=number{
        for x in 0..=number - 1{
            if x == 0 || x == number - 1{
                print!("X ");
            }else if x == (i-1){
                print!("X ");
            }else{
                print!("O ");
            }
            
        }
        println!();
    }

}