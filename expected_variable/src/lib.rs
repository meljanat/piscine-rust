use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let a = compared.to_lowercase();
    let b = expected.to_lowercase();

    let dist = edit_distance(&a, &b);
    let max_len = a.len().max(b.len());

    let similarity = 1.0 - dist as f64 / max_len as f64;

    if similarity > 0.5 {
        let percent = (similarity * 100.0).round() as usize;
        Some(format!("{}%", percent))
    } else {
        None
    }
}
