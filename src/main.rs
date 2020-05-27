mod utils;

fn main() {
    let name = "cary";
    let end = utils::condense(name);
    println!("end: {}", end);
    let final_answer = utils::sum(end);
    println!("final answer {}", final_answer)
}
