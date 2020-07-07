mod utils;

const NAME: &'static str = "cary";
fn main() {
    let numb = utils::condense(NAME);
    println!("end: {}", numb);
    let res = utils::get_reading(numb);
    println!("{}", res);
}
