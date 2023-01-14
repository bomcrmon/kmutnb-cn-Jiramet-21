use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let number:i64=input.trim().parse().expect("Please Type A Number!!!");

    /* iteration */
    println!("Iteration");
    let mut n2:i64 = 0;
    let mut n1:i64 = 1;
    for i in 0..=number {
        let n:i64;
        if i == 0{
            n = n2;
        }else if i == 1{
            n = n1;
        }else{
            n = n1 + n2;
            n2 = n1;
            n1 = n;
        }
        
        if i < number{
           print!("{} , " , n); 
        }else{
            print!("{}" , n);
        }
    }
    println!();

    /* Recursion */
        println!("Recursion");
        for i in 0..=number{
            let a:i64 = re_fibo(i);
            if i < number{
                print!("{} , " , a); 
             }else{
                 print!("{}" , a);
             }
        }
}

fn re_fibo(inp: i64) -> i64{
    if inp == 0 {
        return 0;
    }else if inp == 1 {
        return 1;
    }else{
        return re_fibo(inp - 1) + re_fibo(inp - 2);
    }
}