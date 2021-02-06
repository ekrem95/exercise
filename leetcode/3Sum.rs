impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
       if nums.len() == 0 {
           return vec![];
       }

       let mut nums = nums;
       nums.sort();

       let mut v: Vec<Vec<i32>> = vec![];

       let mut i = 0;
       let limit = nums.len() - 1;
       loop {
           if i >= limit {
               break;
           }

           if i > 0 && nums[i] == nums[i - 1] {
               i += 1;
               continue;
           }

           let mut j = i + 1;
           let mut k = limit;

           loop {
               if j >= k {
                   break;
               }

               if k < limit && nums[k] == nums[k + 1] {
                   k -= 1;
                   continue;
               }

               if nums[i] + nums[j] + nums[k] > 0 {
                   k -= 1;
               } else if nums[i] + nums[j] + nums[k] < 0 {
                   j += 1
               } else {
                   v.push(vec![nums[i], nums[j], nums[k]]);
                   j += 1;
                   k -= 1;
               }
           }
           i += 1;
       }
       v
   }
}