const MINUTES_IN_A_DAY: i32 = 1440;

pub fn to_minutes(time_point: String) -> i32 {
    let sp: Vec<&str> = time_point.split(":").collect();
    let h = sp[0].parse::<i32>().unwrap();
    let m = sp[1].parse::<i32>().unwrap();
    (h * 60 + m) % MINUTES_IN_A_DAY
}

pub fn clockwise(m1: i32, m2: i32) -> i32 {
    if m1 > m2 {
        return m2 + MINUTES_IN_A_DAY - m1;
    }
    m2 - m1
}

pub fn counterclockwise(m1: i32, m2: i32) -> i32 {
    if m1 < m2 {
        return m1 + MINUTES_IN_A_DAY - m2;
    }
    m1 - m2
}

pub fn min(n1: i32, n2: i32) -> i32 {
    if n1 < n2 {
        return n1;
    }
    n2
}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes: Vec<i32> = [].to_vec();
        for (_, e) in time_points.iter().enumerate() {
            minutes.push(to_minutes(e.to_string()));
        }

        let mut r = MINUTES_IN_A_DAY;
        for (i, e) in minutes.iter().enumerate() {
            let mut j = i + 1;
            while j < minutes.len() {
                let m = minutes[j];
                let diff = min(clockwise(*e, m), counterclockwise(*e, m));
                r = min(r, diff);
                j += 1;
            }
        }
        r
    }
}
