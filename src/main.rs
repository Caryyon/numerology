use std::io;

fn main() {

    println!("Hello, what's your name?");

    let mut name = String::new(); // ""

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to get your name.");

    let split_name = name.trim().chars();

    for letter in split_name {
        let letr = letter.to_uppercase();
        let numbr: u8 = letter as u8;
            println!("{}: {}", letr, numbr);
    }

}
