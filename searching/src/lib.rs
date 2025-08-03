pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut reversed = array.to_vec();
    reversed.reverse();
    reversed.iter().position(|&x| x == key)
}
