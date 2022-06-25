impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut nums = nums;
        nums.sort();

        if nums[0] != nums[1] {
            return nums[0];
        }

        let l = nums.len();
        if nums[l - 1] != nums[l - 2] {
            return nums[l - 1];
        }

        let mut i = 1;
        while i < l {
            if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
                return nums[i];
            }
            i += 1;
        }
        0
    }
}