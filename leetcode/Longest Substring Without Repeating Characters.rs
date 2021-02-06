impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut len: usize = 0;
        let bytes = s.as_bytes();
        let limit = bytes.len();

        for i in 0..limit {
            let mut vec: Vec<u8> = [].to_vec();
            for j in i..limit {
                if vec.contains(&bytes[j]) {
                    let l = vec.len();
                    len = if l > len { l } else { len };
                    break;
                } else if j + 1 == limit {
                    let l = vec.len() + 1;
                    len = if l > len { l } else { len };
                }

                vec.push(bytes[j]);
            }
        }

        len as i32
    }
}