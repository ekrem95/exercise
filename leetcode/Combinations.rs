pub fn perm(n: i32, k: i32, start: i32, v: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
    if v.len() == k as usize {
        combinations.push(v.to_vec());
        return;
    }

    for i in start..n + 1 {
        v.push(i);
        perm(n, k, i + 1, v, combinations);
        v.remove(v.len() - 1);
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<i32> = vec![];
        let mut combinations: Vec<Vec<i32>> = vec![];
        perm(n, k, 1, &mut v, &mut combinations);

        combinations
    }
}