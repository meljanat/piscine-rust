pub fn talking(text: &str) -> &str {
    if text.chars().all(|c| c.is_ascii_uppercase() || c == '!') {
        "There is no need to yell, calm down!"
    } else if text.chars().all(|c| c.is_ascii_uppercase() || c == '?') {
        "Quiet, I am thinking!"
    } else if text.chars().any(|c| c.is_ascii_lowercase() || c == '?') {
        "Sure."
    } else if text.is_empty() {
        "Just say something!"
    } else {
        "Interesting"
    }
}
