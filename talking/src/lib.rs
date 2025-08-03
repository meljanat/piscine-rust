pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.chars().all(|c| (c.is_alphabetic() && c.is_ascii_uppercase()) || c == '!') && trimmed.ends_with('!') {
        "There is no need to yell, calm down!"
    } else if trimmed.chars().all(|c| (c.is_alphabetic() && c.is_ascii_uppercase()) || c == '?') && trimmed.ends_with('?') {
        "Quiet, I am thinking!"
    } else if trimmed.chars().any(|c| c.is_ascii_lowercase()) && trimmed.ends_with('?') {
        "Sure."
    } else if trimmed.is_empty() {
        "Just say something!"
    } else {
        "Interesting"
    }
}
