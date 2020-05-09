use std::cmp;
use std::collections::HashMap;
use std::i32;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for num in &nums {
            let counter = map.entry(num).or_insert(0);
            *counter += 1;
            min = cmp::min(min, *num);
            max = cmp::max(max, *num);
        }
        // println!("{} {} {:?}", max, min, map);
        for i in (min..=max).rev() {
            if let Some(x) = map.get(&i) {
                // println!("{} {} {}", i, x, k);
                if k - x <= 0 {
                    return i;
                }
                k = k - x;
            }
        }
        unreachable!();
    }
}
