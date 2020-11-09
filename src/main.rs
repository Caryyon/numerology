mod utils;
use std::io;

fn recall_sum(num: u32) -> Vec<u32> {
    let answer: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    return answer;
}
fn main() {
    let mut name = String::new();
    println!("what name do you align most with? (doesn't have to be your legal name)");
    io::stdin()
        .read_line(&mut name)
        .expect("Sorry didn't get that, might have had some odd symbols in it?");
    let name_as_numb = utils::condense(&name.trim());
    println!("{:?}", name_as_numb);
    let mut sum: u32 = name_as_numb.iter().sum();
    match sum {
        11 | 12 => println!("{}", utils::get_reading(sum)),
        1 => println!("{}", utils::get_reading(sum % 9)),
        2 => println!("{}", utils::get_reading(sum % 9)),
        3 => println!("{}", utils::get_reading(sum % 9)),
        4 => println!("{}", utils::get_reading(sum % 9)),
        5 => println!("{}", utils::get_reading(sum % 9)),
        6 => println!("{}", utils::get_reading(sum % 9)),
        7 => println!("{}", utils::get_reading(sum % 9)),
        8 => println!("{}", utils::get_reading(sum % 9)),
        9 => println!("{}", utils::get_reading(sum % 9)),
        _ => {
            let new_sum = recall_sum(sum);
            return new_sum.iter().sum();
        }
    }
}
