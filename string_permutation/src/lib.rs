pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.chars().collect::<Vec<_>>();
    let mut s2 = s2.chars().collect::<Vec<_>>();
    s1.sort();
    s2.sort();
    s1 == s2
}