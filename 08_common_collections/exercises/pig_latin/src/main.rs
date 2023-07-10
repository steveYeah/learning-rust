fn main() {
    let consonant_word = "first";
    let vowel_word = "apple";

    let word = consonant_word;

    if ['a', 'e', 'i', 'o', 'u'].contains(&word.chars().next().unwrap()) {
        println!("{word}-hay");
    } else {
        let new_word = &word[1..];
        let first_letter = &word.chars().next().unwrap();
        println!("{new_word}-{first_letter}ay");
    }
}
