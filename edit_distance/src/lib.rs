pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let len_source = source_chars.len();
    let len_target = target_chars.len();

    let mut table = vec![vec![0; len_target + 1]; len_source + 1];

    for i in 0..=len_source {
        table[i][0] = i;
    }
    for j in 0..=len_target {
        table[0][j] = j;
    }

    for i in 1..=len_source {
        for j in 1..=len_target {
            if source_chars[i - 1] == target_chars[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                let insert_cost = table[i][j - 1] + 1;
                let delete_cost = table[i - 1][j] + 1;
                let replace_cost = table[i - 1][j - 1] + 1;

                table[i][j] = insert_cost.min(delete_cost.min(replace_cost));
            }
        }
    }

    table[len_source][len_target]
}