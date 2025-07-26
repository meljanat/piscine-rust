pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip = 0;

    for c in s.chars() {
        if skip != 0 && c != '+' && c != '-' {
            skip -= 1;
            continue;
        }
        match c {
            '-' => {
                result.pop();
            }
            '+' => {
                skip += 1;
            }
            _ => {
                result.push(c);
            }
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let bytes = expr.as_bytes();
        let mut op_index = 0;
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'+' || b == b'-' {
                op_index = i;
                break;
            }
        }

        let left: i32 = expr[..op_index].parse().unwrap();
        let right: i32 = expr[op_index + 1..].parse().unwrap();
        let op = expr.as_bytes()[op_index] as char;

        let result = match op {
            '+' => left + right,
            '-' => left - right,
            _ => unreachable!(),
        };

        *expr = result.to_string();
    }
}