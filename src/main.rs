use std::io;

fn main() {
    loop {
        println!("Input number");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("error");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
        break;
    }
}
