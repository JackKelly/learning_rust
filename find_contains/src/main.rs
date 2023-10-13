fn main() {
    let haystack = [
        "foo".to_string(),
        "bar".to_string(),
        "baz".to_string(),
        ];

    let found = find_contains(&haystack, "ba");

    println!("{found:?}");
}

fn find_contains<'a>(haystack: &'a [String], needle: &str) -> Vec<&'a String> {
    let mut found = Vec::new();
    for s in haystack {
        if s.contains(needle) {
            found.push(s);
        }
    }
    found
}