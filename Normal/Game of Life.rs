impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let row_iter = 0..board.len();
        let column_iter = 0..board[0].len();

        for i in row_iter.clone() {
            for j in column_iter.clone() {
                let neighbors = [
                    (i - 1, j),
                    (i - 1, j - 1),
                    (i - 1, j + 1),
                    (i + 1, j),
                    (i + 1, j - 1),
                    (i + 1, j + 1),
                    (i, j - 1),
                    (i, j + 1),
                ]
                .iter()
                .filter(|(a, b)| {
                    row_iter.clone().contains(a)
                        && column_iter.clone().contains(b)
                        && (board[*a][*b] == 1 || board[*a][*b] == 2)
                })
                .count();
                let current = board[i][j];
                if current == 1 {
                    if neighbors < 2 || neighbors > 3 {
                        board[i][j] = 2
                    }
                } else if neighbors == 3 {
                    board[i][j] = 3
                }
            }
        }
        for i in row_iter.clone() {
            for j in column_iter.clone() {
                if board[i][j] == 2 {
                    board[i][j] = 0;
                }
                if board[i][j] == 3 {
                    board[i][j] = 1;
                }
            }
        }
    }
}
