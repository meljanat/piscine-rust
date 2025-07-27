pub fn edit_distance(source: &str, target: &str) -> usize {
    if source == target {
        return 0;
    }
    let mut changes = 0;
    for i in 0..target.len() {
        if target[i] != source[i] {
            changes += 1;
        }
    }
    changes
}