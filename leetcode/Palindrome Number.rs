impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut s: String = "".to_string();
        if x < 0 {
            s += "-";
            s += &x.abs().to_string();
        } else {
            s += &x.to_string();
        }

        s == s.chars().rev().collect::<String>()
    }
}