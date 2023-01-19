use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Enter number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed ReadLine");
    let mut number:i32=input.trim().parse().expect("Enter Number");
    print!("{} =",number);
    let mut count:i32=2;
    'out:loop{
        if number % count ==0{
            'inner:loop{
                if number % count ==0{
                    number /= count;
                    print!("{}",count);
                    if number >1{
                        print!("*");
                    }else{
                        break 'inner;
                    }
                }
                else{
                    break 'inner;
                }
            }
        }
        count += 1;
        if number<=1{
            break 'out;
        }
    }
}


