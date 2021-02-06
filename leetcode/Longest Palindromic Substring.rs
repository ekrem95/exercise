impl Solution {
    pub fn longest_palindrome(s: String) -> String {
       let size = s.len();
       if size < 2 {
           return s;
       }

       let mut longest = "".to_string();
       for i in 0..size {
           let mut j = size;

           while i < j {
               while i < j {
                   if s.as_bytes()[i] as char == s.as_bytes()[j - 1] as char {
                       break;
                   }
                   j -= 1;
               }

               let t = &s[i..j];
               if t.len() < longest.len() {
                   break;
               }
               if t.len() > longest.len() && t == t.chars().rev().collect::<String>() {
                   longest = t.to_string();
                   break;
               }

               j -= 1;
           }
       }

       longest
   }
}