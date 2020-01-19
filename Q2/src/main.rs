mod lib;
use std::io;
fn main() {
    loop {
        println!("Please input your guess.");

        let mut input1 = String::new();

        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");

        let input1: u32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", input1);

        lib::welcome::pakistan::table(input1);

        break;
    }
}
