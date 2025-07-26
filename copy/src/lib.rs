pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .map(|num| {
            let n: f64 = num.parse().unwrap();
            n.exp().to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");
    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b
        .iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect::<Vec<f64>>();
    (b, logs)
}