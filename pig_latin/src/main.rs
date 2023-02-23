fn main() {
    let sentence = "this is a first test";

    for word in sentence.split_whitespace() {
        println!("{}", word_to_pig_latin(&word));
    }
}


fn word_to_pig_latin(word: &str) -> String {
    let first_letter = word.chars().nth(0).unwrap();
    if is_vowel(&first_letter) {
        format!("{word}-hay")
    } else {
        format!(
            "{}-{}ay",
            String::from_iter(word.chars().skip(1)),
            first_letter,
        )
    }
}


fn is_vowel(char: &char) -> bool {
    const VOWELS: &str = "aeiou";
    VOWELS.chars().any(|c| c==*char)
}