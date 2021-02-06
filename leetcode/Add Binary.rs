impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
      let mut v: Vec<u32> = vec![];
  
      let mut a = a;
      let mut b = b;
      let diff = a.len() as i32 - b.len() as i32;
      if diff > 0 {
          b = "0".repeat(diff as usize) + &b;
      } else if diff < 0 {
          a = "0".repeat(diff.abs() as usize) + &a;
      }
  
      let mut on_hand = false;
  
      let mut i = a.len() - 1;
      while i >= 0 {
          let mut n = 0;
  
          if a.len() > i {
              let b: u8 = a.as_bytes()[i];
              let c: char = b as char;
              if c == '1' {
                  n += 1;
              }
          }
          if b.len() > i {
              let b: u8 = b.as_bytes()[i];
              let c: char = b as char;
              if c == '1' {
                  n += 1;
              }
          }
  
          if on_hand == false {
              if n == 0 {
                  v.push(0);
              } else if n == 1 {
                  v.push(1);
              } else {
                  v.push(0);
                  on_hand = true;
              }
          } else {
              if n == 0 {
                  v.push(1);
                  on_hand = false;
              } else if n == 1 {
                  v.push(0);
              } else {
                  v.push(1);
                  on_hand = true;
              }
          }
  
          if i == 0 {
              break;
          }
          i -= 1;
      }
  
  
      if on_hand {
          v.push(1);
      }
  
      v.reverse();
  
      v.iter().map(|&x| x.to_string()).collect()
  }
  }