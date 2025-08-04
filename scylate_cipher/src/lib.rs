pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return String::new(); // Avoid division by zero
    }
    
    let len = message.len();
    let mut padded = message.to_string();

    // Pad with spaces if needed
    let rem = len % i;
    if rem != 0 {
        padded.extend(std::iter::repeat(' ').take(i - rem));
    }

    let rows = padded.len() / i;
    let chars: Vec<char> = padded.chars().collect();
    let mut result = String::with_capacity(padded.len());

    // Read column-wise
    for col in 0..i {
        for row in 0..rows {
            result.push(chars[row * i + col]);
        }
    }

    result
}