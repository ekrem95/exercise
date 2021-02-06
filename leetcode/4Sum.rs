impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
       let mut res: Vec<Vec<i32>> = vec![];
       if nums.len() < 4 {
           return res;
       }

       let mut nums = nums;
       nums.sort();

       for i in 0..nums.len() {
           if i != 0 && nums[i] == nums[i - 1] {
               continue;
           }
           for j in i + 1..nums.len() - 2 {
               if j != i + 1 && nums[j] == nums[j - 1] {
                   continue;
               }

               let mut k = j + 1;
               let mut l = nums.len() - 1;
               while k < l {
                   let sum = nums[i] + nums[k] + nums[l] + nums[j];
                   if sum < target {
                       k += 1;
                   } else if sum > target {
                       l -= 1;
                   } else {
                       let mut candidate = vec![nums[i], nums[k], nums[l], nums[j]];
                       candidate.sort();
                       if !res.contains(&candidate) {
                           res.push(candidate);
                       }
                       k += 1;
                       l -= 1;

                       while k < l && nums[l] == nums[l + 1] {
                           l -= 1;
                       }

                       while k < l && nums[k] == nums[k - 1] {
                           k += 1;
                       }
                   }
               }
           }
       }

       res
   }
}