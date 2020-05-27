mod convert_to_number;

fn condense(name: &str) -> u32 {
    let mut letters_to_numbers = 0;
    for letter in name.to_uppercase().chars() {
        let number = convert_to_number::convert(&letter.to_string());
        letters_to_numbers = letters_to_numbers + number;
        println!("{}", number)
    }
    return letters_to_numbers
}

fn main() {
    let name = "cary";
    let end = condense(name);
    println!("end: {}", end);
    let final_answer = convert_to_number::sum(end);
    println!("final answer {}", final_answer)
}
