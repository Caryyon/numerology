mod utils;
use std::io;

fn main() {
    let mut name = String::new();
    println!("what is your name?");
    io::stdin().read_line(&mut name).expect("Didn't get that");
    let name_as_numb = utils::condense(&name.trim());
    let sum: u32 = name_as_numb.iter().sum();
    if (sum == 11) | (sum == 22) {
        let special_numerology_number = utils::get_reading(sum);
        println!("{}", special_numerology_number);
    } else {
        let numerology_number = utils::get_reading(sum % 9);
        println!("{}", numerology_number);
    }
}
