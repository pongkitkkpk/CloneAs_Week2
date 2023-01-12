use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed ReadLine");
    let mom:i32=input.parse().expect("Enter Number");
    
    for i in (0..=mom-1){
        for _ in 0..=i{
            print!("* ");
        }
        println!();
    }
    
    }

