fn first_fifty_even_square() -> Vec<i32> {
    (1..=50).map(|n| (2 * n as i32).pow(2)).collect()
}