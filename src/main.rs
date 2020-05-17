use std::io;

// need a constant for the numbers the letters equate to


fn main() {

    // ask the user to put in their name
    println!("=====================================");
    println!("what is the name you prefer to go by?");
    println!("=====================================");

    // setup a var with an empty string
    let mut name = String::new();

    // get the users name
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to get your name."); // if things go wrong with getting their name

    // trim off the \n and split the name into seperate characters
    let split_name = name.trim().chars();

    // loop over the characters
    for letter in split_name {
        let letr = letter.to_uppercase(); // change it to upper case
            println!("The letter {} of your name", letr);
    }

}
