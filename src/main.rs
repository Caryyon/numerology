mod utils;

const NAME: &'static str = "cary wolff";
fn main() {
    println!("what is your name?");
    let name_as_numb = utils::condense(NAME);
    let sum: u32 = name_as_numb.iter().sum();
    if (sum == 11) | (sum == 12) {
        let special_numerology_number = utils::get_reading(sum);
        println!("{}", special_numerology_number);
    } else {
        let numerology_number = utils::get_reading(sum % 9);
        println!("{}", numerology_number);
    }
}
