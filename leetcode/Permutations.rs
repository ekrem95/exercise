impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len() {
            let v = vec![nums[i]];
            combinations.push(v);
        }

        while combinations[combinations.len() - 1].len() != nums.len() {
            let current = combinations.clone();

            for c in current {
                for i in 0..nums.len() {
                    if c.contains(&nums[i]) {
                        continue;
                    }

                    let mut nc = c.clone();
                    nc.push(nums[i]);
                    if !combinations.contains(&nc) {
                        combinations.push(nc);
                    }
                }
            }
        }

        combinations
            .into_iter()
            .filter(|e| e.len() == nums.len())
            .collect()
    }
}