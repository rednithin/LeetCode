use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            if a != b {
                heap.push(a - b);
            }
        }

        match heap.pop() {
            Some(x) => x,
            None => 0,
        }
    }
}
