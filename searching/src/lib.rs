pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.to_vec().reverse().iter().position(|&x| x == key)
}
