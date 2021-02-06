impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let index: i32 = match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        };
        index
    }
}