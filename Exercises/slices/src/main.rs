/**
 * Write a function that takes a string of words separated by spaces and returns the first word it finds in
 * that string. If the function doesn’t find a space in the string, the whole string must be one word, so the
 * entire string should be returned.
 */
fn main() {
    let string_input = String::from("hehehehe");
    let word = find_first_word(&string_input);
    let word2 = find_first_word_better_signature(&string_input[..]);
    println!("result: {word},{word2}");

    let string_literal = "testing literal";
    //let word3 = find_first_word(&string_literal); <-- This throws
    let word3 = find_first_word_better_signature(string_literal);
    println!("result: {word3}")
}

/// Finds the first word in a text input
fn find_first_word(text_input: &String) -> &str {
    let bytes = text_input.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text_input[0..i];
        }
    }

    &text_input[..]
}

/// This works for both &String and &str types!
fn find_first_word_better_signature(text_input: &str) -> &str {
    let bytes = text_input.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text_input[0..i];
        }
    }

    &text_input[..]
}
