use levenshtein::levenshtein;

pub fn edit_distance(source: &str, target: &str) -> usize {
    levenshtein(source, target)
}