impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count: usize = 0;
        for (pos, e) in nums.clone().iter().enumerate() {
            if *e == val {
                nums.remove(pos - count);
                count += 1;
            }
        }
        nums.len() as i32
    }
}