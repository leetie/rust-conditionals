use console::style;
use console::Term;
use std::io::Write;
use std::{io, thread, time};

fn main() {
    let term = Term::stdout();
    term.clear_screen().expect("failed to clear");
    loop {
        println!(
            "{} {}",
            style("Please input a number").black().on_white(),
            style('🔮').white()
        );
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("error");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // if number < 5 {
        //     println!("condition was true");
        // } else {
        //     println!("condition was false");
        // }
        print!("I am sensing");
        let mut counter = 4;
        while counter > 0 {
            print!(".");
            io::stdout().flush().expect("?????");
            thread::sleep(time::Duration::from_millis(500));
            counter -= 1;
        }
        print!("\n");
        if number == 0 {
            println!("that the number is not divisible");
            break;
        };
        if number % 4 == 0 {
            println!("that the number is divisible by 4");
        } else if number % 3 == 0 {
            println!("that the number is divisible by 3");
        } else if number % 2 == 0 {
            println!("that the number is divisible by 2");
        } else {
            println!("that the number is not divisible by 4, 3, or 2");
        }
        break;
    }
}
