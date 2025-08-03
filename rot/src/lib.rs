pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let pos = c as u8 - base;
                let rotated = (26 + (pos as i8 + key % 26)) % 26;
                (base + rotated as u8) as char
            } else {
                c
            }
        })
        .collect()
}