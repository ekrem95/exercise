impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let mut res: Vec<Vec<i32>> = [].to_vec();
        res.push(intervals.first().unwrap().to_vec());

        let mut i = 0;
        for v in &intervals[1..] {
            if v[0] <= res[i][1] {
                if v[1] > res[i][1] {
                    res[i][1] = v[1];
                }
            } else {
                res.push([v[0], v[1]].to_vec());
                i += 1;
            }
        }
        res
    }
}