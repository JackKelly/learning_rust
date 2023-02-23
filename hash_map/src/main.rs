use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {team_name} = {score}");

    for (key, value) in &scores {
        println!("{key}={value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // ******************

    let text = "hello world wonderful world";

    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count_for_word = word_counts.entry(word).or_insert(0);
        *count_for_word += 1;
    }

    println!("{:#?}", word_counts);

}
