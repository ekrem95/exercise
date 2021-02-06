impl Solution {
    pub fn my_atoi(my_str: String) -> i32 {
        let mut f_str: String = "".to_string();
        for (i, c) in my_str.chars().enumerate() {
            if c.is_digit(10) {
                f_str.push_str(&c.to_string());
                continue;
            }
            if f_str != "" {
                break;
            }
            if c == ' ' {
                continue;
            }
            if c == '-' || c == '+' {
                f_str = c.to_string();
                continue;
            }
            return 0;
        }

        match f_str.parse::<i32>() {
            Ok(f) => f as i32,
            Err(e) => {
                let e_str: &str = &e.to_string();
                match e_str {
                    "number too small to fit in target type" => std::i32::MIN,
                    "number too large to fit in target type" => std::i32::MAX,
                    _ => 0,
                }
            }
        }
    }
}