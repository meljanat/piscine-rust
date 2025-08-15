pub fn get_diamond(c: char) -> Vec<String> {
    let max_index = c as u8 - b'A';
    let size = (max_index * 2 + 1) as usize;
    let mut diamond = Vec::with_capacity(size);

    for i in 0..=max_index {
        let letter = (b'A' + i) as char;
        let outer_spaces = (max_index - i) as usize;
        let inner_spaces = if i == 0 { 0 } else { (i * 2 - 1) as usize };

        let mut row = String::new();
        row.push_str(&" ".repeat(outer_spaces));
        row.push(letter);
        if i > 0 {
            row.push_str(&" ".repeat(inner_spaces));
            row.push(letter);
        }
        row.push_str(&" ".repeat(outer_spaces));

        diamond.push(row);
    }

    for i in (0..max_index).rev() {
        diamond.push(diamond[i as usize].clone());
    }

    diamond
}
