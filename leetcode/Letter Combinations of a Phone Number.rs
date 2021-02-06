impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
       if digits == "" {
           return vec![];
       }

       let mut all: Vec<Vec<String>> = vec![];

       for c in digits.chars() {
           let current: Vec<String> = match c {
               '2' => vec!["a".to_string(), "b".to_string(), "c".to_string()],
               '3' => vec!["d".to_string(), "e".to_string(), "f".to_string()],
               '4' => vec!["g".to_string(), "h".to_string(), "i".to_string()],
               '5' => vec!["j".to_string(), "k".to_string(), "l".to_string()],
               '6' => vec!["m".to_string(), "n".to_string(), "o".to_string()],
               '7' => vec![
                   "p".to_string(),
                   "q".to_string(),
                   "r".to_string(),
                   "s".to_string(),
               ],
               '8' => vec!["t".to_string(), "u".to_string(), "v".to_string()],
               '9' => vec![
                   "w".to_string(),
                   "x".to_string(),
                   "y".to_string(),
                   "z".to_string(),
               ],
               _ => vec![],
           };

           all.push(current);
       }

       let mut combinations: Vec<String> = vec![];
       for e in &all[0] {
           combinations.push(e.to_string());
       }

       let mut i = 1;
       while i < all.len() {
           let current = &all[i];
           let cih = combinations.clone();

           for e in current {
               for s in &cih {
                   combinations.push(format!("{}{}", s, e.to_string()));
               }
           }

           i += 1;
       }

       combinations
           .iter()
           .filter(|e| e.len() == digits.len())
           .cloned()
           .collect()
   }
}