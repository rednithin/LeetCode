use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut maximum = 0;
        while i < j {
            let area = (j as i32 - i as i32) * cmp::min(height[i], height[j]);
            maximum = cmp::max(maximum, area);
            if height[i] <= height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        maximum
    }
}
