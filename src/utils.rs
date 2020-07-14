// break down letters into numbers
pub fn condense(name: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let caps = name.to_uppercase();
    for letter in caps.chars() {
        let number = letter_to_number(&letter.to_string());
        numbers.push(number);
    }
    numbers
}

// gives back your final sum number meaning.
pub fn get_reading(number: u32) -> &'static str {
    let final_number = match number {
        0 => "you are a 0",
        1 => "you are a 1",
        2 => "you are a 2",
        3 => "you are a 3",
        4 => "you are a 4",
        5 => "you are a 5",
        6 => "you are a 6",
        7 => "you are a 7",
        8 => "you are a 8",
        9 => "you are a 9",
        11 => "you are a 11",
        12 => "you are a 12",
        _ => "i don't know what you are doing",
    };
    return final_number;
}

// pass it a capital letter string and it will return a int
pub fn letter_to_number(letter: &str) -> u32 {
    let number = match letter {
        "A" | "J" | "S" => 1,
        "B" | "K" | "T" => 2,
        "C" | "L" | "U" => 3,
        "D" | "M" | "V" => 4,
        "E" | "N" | "W" => 5,
        "F" | "O" | "X" => 6,
        "G" | "P" | "Y" => 7,
        "H" | "Q" | "Z" => 8,
        "I" | "R" => 9,
        &_ => 0,
    };
    return number;
}

#[test]
fn letter_to_number_test() {
    assert_eq!(letter_to_number("A"), 1);
    assert_eq!(letter_to_number("B"), 2);
    assert_eq!(letter_to_number("C"), 3);
}
