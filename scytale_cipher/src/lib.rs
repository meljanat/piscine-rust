pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return String::new();
    }
    
    let len = message.len();
    let mut padded = message.to_string();

    let rem = len % i;
    if rem != 0 {
        padded.extend(std::iter::repeat(' ').take(i - rem));
    }

    let rows = padded.len() / i;
    let chars: Vec<char> = padded.chars().collect();
    let mut result = String::with_capacity(padded.len());

    for col in 0..i {
        for row in 0..rows {
            result.push(chars[row * i + col]);
        }
    }

    result
}