impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pos: Vec<i32> = vec![-1, -1];
    
        let l = nums.len();
        if l == 0 {
            return pos;
        }
    
        let mut i: usize = 0;
        let mut j: usize = l - 1;
        while i <= j {
            if nums[i] == target {
                if pos[0] == -1 || i < pos[0] as usize {
                    pos[0] = i as i32;
                }
    
                if pos[1] == -1 {
                    pos[1] = i as i32;
                }
            }
    
            if nums[j] == target {
                if pos[0] == -1 {
                    pos[0] = j as i32;
                }
    
                if pos[1] == -1 || j > pos[1] as usize {
                    pos[1] = j as i32;
                }
            }
    
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    
        if pos[0] != -1 {
            let mut k: usize = pos[0] as usize;
    
            while k > 0 {
                k -= 1;
    
                if nums[k] == nums[pos[0] as usize] {
                    pos[0] = k as i32;
                } else if nums[k] < pos[0] {
                    break;
                }
            }
        }
    
        if pos[1] != -1 {
            let mut k: usize = pos[1] as usize;
    
            while k < l - 1 {
                k += 1;
    
                if nums[k] == nums[pos[1] as usize] {
                    pos[1] = k as i32;
                } else if nums[k] > pos[0] {
                    break;
                }
            }
        }
    
        pos
    }
}