pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    input[..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut to_cap = true;
    input.chars()
        .map(|c| {
            if c.is_whitespace() {
                to_cap = true;
                c.to_string()
            } else if to_cap {
                to_cap = false;
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| 
        if c.is_uppercase() {
            c.to_lowercase().to_string()
        } else {
            c.to_uppercase().to_string()
        })
        .collect::<String>()
}