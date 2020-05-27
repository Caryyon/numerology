pub fn condense(name: &str) -> u32 {
    let mut letters_to_numbers = 0;
    for letter in name.to_uppercase().chars() {
        let number = convert(&letter.to_string());
        letters_to_numbers = letters_to_numbers + number;
        println!("{}", number)
    }
    return letters_to_numbers
}
pub fn sum(number: u32) -> &'static str {
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
        22 => "you are a 12",
        _ =>  "i don't know what you are doing"
    };
    println!("{:?}", final_number);
    return final_number
}
pub fn convert(letter: &str) -> u32 {
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
            &_ => 0
        };
        return number
}

#[test]
fn convert_test_letter() {
    assert_eq!(convert("A"), 1);
    assert_eq!(convert("B"), 2);
}
#[test]
fn convert_test_number() {
    assert_eq!(convert(1), 1)
}