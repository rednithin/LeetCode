use std::cmp::min;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp_table = vec![vec![i32::max_value(); (amount + 1) as usize]; coins.len()];

        for i in 0..coins.len() {
            for j in 0..amount + 1 {
                if j == 0 {
                    dp_table[i as usize][j as usize] = 0;
                    continue;
                }
                let minimum = if i == 0 {
                    i32::max_value()
                } else {
                    dp_table[(i - 1) as usize][j as usize]
                };
                dp_table[i as usize][j as usize] = minimum;
                if j as i32 >= coins[i] {
                    let value = dp_table[i as usize][(j - coins[i]) as usize];
                    if value != i32::max_value() {
                        dp_table[i as usize][j as usize] = min(value + 1, minimum);
                    }
                }
            }
        }
        // println!("{:?}", dp_table);
        let value = dp_table[coins.len() - 1 as usize][amount as usize];
        if value != i32::max_value() {
            value
        } else {
            -1
        }
    }
}
