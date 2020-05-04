use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for na in &a {
            for nb in &b {
                let sum = na + nb;
                if let Some(x) = map.get_mut(&sum) {
                    *x = *x + 1;
                } else {
                    map.insert(sum, 1);
                }
            }
        }
        let mut sum = 0;
        for nc in &c {
            for nd in &d {
                if let Some(x) = map.get(&-(nc + nd)) {
                    sum += *x;
                }
            }
        }
        // println!("{:?}", map);
        sum
    }
}
