use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed ReadLine");
    let mom:i32=input.trim().parse().expect("Enter Number");
    
    
    for i in 1..=mom{
        for x in 0..=mom - 1{
            if x == 0 || x == mom - 1{
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