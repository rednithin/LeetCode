use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn find_minimum_path(
        grid: &Vec<Vec<i32>>,
        hashmap: &mut HashMap<(usize, usize), i32>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> i32 {
        if let Some(x) = hashmap.get(&(i, j)) {
            return *x;
        }
        if i == 0 && j == 0 {
            return grid[i][j];
        } else if !(0..m).contains(&i) || !(0..n).contains(&j) {
            return i32::max_value();
        };
        let first = Solution::find_minimum_path(grid, hashmap, i - 1, j, m, n);
        let second = Solution::find_minimum_path(grid, hashmap, i, j - 1, m, n);
        let curr = grid[i][j] + std::cmp::min(first, second);
        hashmap.insert((i, j), curr);
        // println!(
        //     "i:{} j:{} first:{} second:{} curr:{}",
        //     i, j, first, second, curr
        // );
        return curr;
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        };
        let n = grid[0].len();
        let mut hashMap: HashMap<(usize, usize), i32> = HashMap::new();
        Solution::find_minimum_path(&grid, &mut hashMap, m - 1, n - 1, m, n)
    }
}
