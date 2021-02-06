impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let default = "".to_string();
    let first = strs.first().unwrap_or(&default).to_string();

    if first == default || strs.len() == 1 {
        return first;
    }

    let mut updated = false;
    let mut count = 0;
    for s in &strs[1..] {
        if s == "" {
            count = 0;
            continue;
        }
        let mut ic = 0;
        for (i, c) in first.chars().enumerate() {
            if s.len() - 1 == i || s.chars().nth(i).unwrap() != c {
                if s.chars().nth(i).unwrap() == c {
                    ic += 1;
                }
                break;
            }

            ic += 1;
        }
        if !updated || ic < count {
            count = ic;
            updated = true;
        }
    }

    if count != 0 {
        return first[..count].to_string();
    }
    "".to_string()
    }
}