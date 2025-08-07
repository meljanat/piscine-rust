pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split(' ')
        .filter_map(|word| word.parse::<u32>().ok())
        .map(|num| Box::new(num))
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
