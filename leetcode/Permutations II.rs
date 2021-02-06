impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len() {
            let v = vec![i as i32];
            combinations.push(v);
        }

        while combinations[combinations.len() - 1].len() != nums.len() {
            let current = combinations.clone();

            for c in &current {
                for i in 0..nums.len() {
                    if c.contains(&(i as i32)) {
                        continue;
                    }

                    let mut nc = c.clone();
                    nc.push(i as i32);
                    combinations.push(nc);
                }
            }

            combinations = (&combinations[current.len()..]).to_vec();
        }

        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..combinations.len() {
            let mut c = combinations[i].clone();
            for j in 0..c.len() {
                c[j] = nums[c[j] as usize];
            }
            if !result.contains(&c) {
                result.push(c);
            }
        }
        result
    }
}