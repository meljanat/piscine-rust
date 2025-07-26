pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .map(|part| {
                    part.chars().next().unwrap_or_default().to_uppercase().to_string() + "."
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect()
}