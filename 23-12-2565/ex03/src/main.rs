use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed ReadLine");
    let mom:i32=input.trim().parse().expect("Enter Number");
    
    for i in (1..=mom){
        for a in 0..=i-1{
            print!("  ");
        }
        for a in 0..=(mom-i)*(2){
            print!("* ");
        }
        println!();
    }
    
    }

