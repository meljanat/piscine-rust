pub fn edit_distance(source: &str, target: &str) -> usize {
    if source == target {
        return 0;
    }

    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let min_len = source_chars.len().min(target_chars.len());
    let mut changes = 0;

    for i in 0..min_len {
        if source_chars[i] != target_chars[i] {
            changes += 1;
        }
    }

    changes + (source_chars.len().max(target_chars.len()) - min_len)
}