impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let size = nums.len();
        let mut f = nums[0] + nums[1] + nums[nums.len() - 1];

        for i in 0..size {
            let mut j = i + 1;
            let mut k = size - 1;

            loop {
                if j >= k {
                    break;
                }

                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return sum;
                }

                if (target - sum).abs() < (target - f).abs() {
                    f = sum;
                }

                if sum > target {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        f
    }
}