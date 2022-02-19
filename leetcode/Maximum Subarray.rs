impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = 0;
        for n in nums {
            if sum < 0 {
                sum = 0;
            }
            sum += n;
            max = if max > sum { max } else { sum };
        }
        max
    }
}
