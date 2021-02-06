impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut first = true;
        let mut count: usize = 0;
        let mut last: i32 = 0;

        for (pos, e) in nums.clone().iter().enumerate() {
            if *e != last || first {
                last = *e;
            } else {
                nums.remove(pos - count);
                count += 1
            }
            first = false;
        }
        nums.len() as i32
    }
}