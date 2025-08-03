pub fn talking(text: &str) -> &str {
    text = text.trim();
    if text.chars().all(|c| (c.is_alphabetic() && c.is_ascii_uppercase()) || c == '!') && text.ends_with('!') {
        "There is no need to yell, calm down!"
    } else if text.chars().all(|c| (c.is_alphabetic() && c.is_ascii_uppercase()) || c == '?') && text.ends_with('?') {
        "Quiet, I am thinking!"
    } else if text.chars().any(|c| c.is_ascii_lowercase()) && text.ends_with('?') {
        "Sure."
    } else if text.is_empty() {
        "Just say something!"
    } else {
        "Interesting"
    }
}
