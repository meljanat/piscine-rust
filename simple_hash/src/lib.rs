use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut frequency_map = HashMap::new();
    for &word in words {
        let count: &mut usize = frequency_map.entry(word).or_insert(0);
        *count += 1;
    }
    frequency_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}