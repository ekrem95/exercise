use std::str::FromStr;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut v = x.to_string().chars().collect::<String>();
        let mut sign = "".to_string();
        
        if !v.chars().all(char::is_alphanumeric) {
            sign = v.chars().next().unwrap().to_string();

            v = v
                .char_indices()
                .next()
                .and_then(|(i, _)| v.get(i + 1..))
                .unwrap_or("")
                .to_string();
        }

        let s = sign + &v.chars().rev().collect::<String>();
        i32::from_str(&s).unwrap_or(0)
    }
}