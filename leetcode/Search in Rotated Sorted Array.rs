impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
      let mut i = 0;
      let mut j = nums.len() - 1;
  
      while i <= j {
          if nums[i] == target {
              return i as i32;
          }
          if nums[j] == target {
              return j as i32;
          }
  
          i += 1;
          if j > 0 {
              j -= 1;
          }
      }
  
      -1
    }
  }