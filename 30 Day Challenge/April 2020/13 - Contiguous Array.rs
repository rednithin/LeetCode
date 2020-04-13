pub use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut maxLength = 0;
        let mut count = 0;

        for (index, num) in nums.iter().enumerate() {
            count += if *num == 1 { 1 } else { -1 };
            if let Some(x) = map.get(&count) {
                let diff = index as i32 - x;
                if diff > maxLength {
                    maxLength = diff;
                }
            } else {
                map.insert(count, index as i32);
            }
        }

        maxLength
    }
}
