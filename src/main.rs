mod utils;

const NAME: &'static str = "cary";
fn main() {
    let numb = utils::condense(NAME);
        println!("end: {}", numb);
        if (numb < 10) | (numb == 11) | (numb == 22) {
            let final_number = utils::get_reading(numb);
            println!("final_number {}", final_number);
        }
}
