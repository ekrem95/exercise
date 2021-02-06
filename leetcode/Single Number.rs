impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut nums = nums;
        nums.sort();

        let l = nums.len() - 1;

        let mut i = 1;
        let mut j = l - 1;

        if nums[0] != nums[1] {
            return nums[0];
        }
        if nums[l] != nums[j] {
            return nums[l];
        }

        while i < j {
            if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
                return nums[i];
            }
            if nums[j] != nums[j - 1] && nums[j] != nums[j + 1] {
                return nums[j];
            }

            i += 1;
            j -= 1;
        }

        0
    }
}