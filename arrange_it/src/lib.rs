pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort_by_key(|w| w.chars().find(|c| c.is_ascii_digit()).unwrap());
    let mut result: Vec<String> = Vec::new();

    for w in words {
        let cleaned: String = w.chars().filter(|c| !c.is_ascii_digit()).collect();
        result.push(cleaned);
    }

    result.join(" ")
}