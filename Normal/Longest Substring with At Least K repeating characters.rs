use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn recursive(s: &str, k: i32) -> i32 {
        if s.len() < 0 {
            return 0;
        }
        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map.iter()
            .filter(|&(_, &freq)| freq < k)
            .map(|(&a, _)| a)
            .max()
            .map(|x| {
                s.split(|c| c == x)
                    .map(|new| Solution::recursive(new, k))
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(s.len() as i32)
    }
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Solution::recursive(&s, k)
    }
}
