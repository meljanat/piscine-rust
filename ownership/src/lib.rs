pub fn first_subword(mut s: String) -> String {
    let mut first = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            first.push(c);
            continue;
        }
        if c.is_uppercase() || c == '_' {
            break;
        }
        first.push(c);
    }
    first
}