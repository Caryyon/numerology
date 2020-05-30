mod utils;

fn main() {
    let name = "cary";
    let numb = utils::condense(name);
        println!("end: {}", numb);
        if (numb < 10) | (numb == 11) | (numb == 22) {
            let final_answer = utils::get_reading(numb);
            println!("final answer {}", final_answer);
        }
}
