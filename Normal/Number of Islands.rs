impl Solution {
    pub fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if (0..grid.len()).contains(&i) && (0..grid[0].len()).contains(&j) {
            if grid[i][j] == '1' {
                grid[i][j] = '0';
                Solution::dfs(grid, i - 1, j);
                Solution::dfs(grid, i + 1, j);
                Solution::dfs(grid, i, j - 1);
                Solution::dfs(grid, i, j + 1);
            }
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Solution::dfs(&mut grid, i, j);
                }
            }
        }
        count
    }
}
