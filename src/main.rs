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
        break;
    }
}
