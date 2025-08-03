pub fn is_pangram(sentence: &str) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters.chars().all(|c| sentence.to_ascii_lowercase().contains(c))
}