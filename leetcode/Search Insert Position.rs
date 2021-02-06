impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut length = nums.len();
        if length == 0 {
            return 0;
        } else {
            length -= 1;
        }

        let mut initialised: bool = false;
        let mut insert_index: i32 = 0;

        for (pos, e) in nums.iter().enumerate() {
            if *e == target {
                return pos as i32;
            }
            if !initialised {
                if target < *e {
                    initialised = true;
                    insert_index = pos as i32
                } else if pos == length {
                    return pos as i32 + 1;
                }
            }
        }
        insert_index
    }
}