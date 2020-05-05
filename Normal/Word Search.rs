use std::collections::HashSet;

impl Solution {
    pub fn rec(
        board: &Vec<Vec<char>>,
        word: &String,
        set: &mut HashSet<(i32, i32)>,
        i: i32,
        j: i32,
        k: i32,
    ) -> bool {
        // println!("{:?} {:?} {} {} {} ", word, set, i, j, k);
        if k == word.len() as i32 {
            return true;
        }
        if !(0..board.len()).contains(&(i as usize)) || !(0..board[0].len()).contains(&(j as usize))
        {
            return false;
        } else {
            if set.get(&(i, j)).is_none()
                && word.chars().nth(k as usize).unwrap() == board[i as usize][j as usize]
            {
                set.insert((i, j));
                let answer = [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
                    .iter()
                    .any(|(a, b)| Solution::rec(&board, &word, set, *a, *b, k + 1));
                if answer {
                    return true;
                } else {
                    set.remove(&(i, j));
                }
            }
        }
        false
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::rec(&board, &word, &mut HashSet::new(), i as i32, j as i32, 0) {
                    return true;
                }
            }
        }
        false
    }
}
