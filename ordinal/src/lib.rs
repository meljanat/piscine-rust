pub fn num_to_ordinal(x: u32) -> String {
    let str_x = x.to_string();
    let suffix = match str_x.as_str().chars().last() {
        Some('1') => "st",
        Some('2') => "nd",
        Some('3') => "rd",
        _ => "th",
    };
    format!("{}{}", x, suffix)
}
