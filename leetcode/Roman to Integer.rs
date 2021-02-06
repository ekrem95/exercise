impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s == "" {
            return 0;
        }

        let v: Vec<char> = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];

        let size = s.len() - 1;
        let mut i = size;
        let mut n: i32 = 0;
        let mut last_index: i32 = 1;

        loop {
            let b: u8 = s.as_bytes()[i];
            let c = b as char;

            let index: i32 = v.iter().position(|&r| r == c).unwrap() as i32;

            let val = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if i < size && index < last_index {
                n -= val;
            } else {
                n += val;
            }

            if i == 0 {
                return n;
            }
            i -= 1;
            last_index = index;
        }
    }
}