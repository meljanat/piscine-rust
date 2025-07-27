use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    sorted[sorted.len() / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();
    for &item in list {
        *occurrences.entry(item).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap_or(0)
}