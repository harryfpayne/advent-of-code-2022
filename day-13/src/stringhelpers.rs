pub fn find_first_bracket_pair(s: &String) -> Option<(usize, usize)> {
    let mut stack = vec![];
    for (i, x) in s.chars().enumerate() {
        match x {
            '[' => {
                stack.push(i);
            }
            ']' => {
                let matching_start_backet = stack.pop().expect("] before [");
                if stack.len() == 0 {
                    return Some((matching_start_backet, i))
                }
            }
            _ => continue
        }
    }

    None
}

pub fn split_range(s: &String, start: usize, end: usize) -> String {
    let (_, first_part) = s.split_at(start);
    let (inner_part, _) = first_part.split_at(end - start + 1);
    inner_part.to_string()
}

pub fn strip_brackets(s: &String) -> String {
    if !s.starts_with("[") || !s.ends_with("]") {
        panic!("value isn't wrapped by []: {}", s)
    }

    let (inner, _) = s.split_once("[").unwrap().1.rsplit_once("]").unwrap();
    inner.to_string()
}

pub fn replace_substring(s: &String, start: usize, end: usize, c: char) -> String {
    let range = start..end+1;
    let mut new_str = String::new();

    for (i, existing_char) in s.chars().enumerate() {
        if range.contains(&i) {
            new_str.push(c);
            continue
        }
        new_str.push(existing_char);
    }

    new_str
}