use std::collections::HashMap;

impl Solution {
pub fn is_valid(s: String) -> bool {
    let mut mymap = HashMap::new();
    mymap.insert("(".to_string(), ")".to_string());
    mymap.insert("{".to_string(), "}".to_string());
    mymap.insert("[".to_string(), "]".to_string());

    let mut a: Vec<String> = [].to_vec();
    let mut b: Vec<String> = [].to_vec();

    for (_, c) in s.chars().enumerate() {
        let v = mymap.entry(c.to_string()).or_default().to_string();
        if v != "" {
            a.push(c.to_string());
            b.push(v.to_string());
            continue;
        }

        if a.len() != b.len() || a.len() == 0 {
            return false;
        }

        let new_len = a.len() - 1;

        for removed_element_a in a.drain(new_len..) {
            let got: String = mymap
                .entry(removed_element_a.clone())
                .or_default()
                .to_string();

            if c.to_string() != got {
                return false;
            }

            b.drain(new_len..);
        }
    }

    a.len() == 0
}
}