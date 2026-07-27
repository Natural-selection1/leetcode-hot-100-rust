/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N 皇后
 */

// @lc code=start
const QUEEN: char = 'Q';
const EMPTY: char = '.';
type Vec2D<T> = Vec<Vec<T>>;

impl crate::Solution {
    pub fn solve_n_queens(size: i32) -> Vec2D<String> {
        let size = size as usize;
        let mut result = vec![];
        dfs(0, size, &mut vec![vec![EMPTY; size]; size], &mut result);
        result
    }
}

fn dfs(row: usize, size: usize, chessboard: &mut Vec2D<char>, result: &mut Vec2D<String>) {
    if row == size {
        return result.push(chessboard.iter().map(|row| row.iter().collect()).collect());
    }

    for col in 0..size {
        if is_valid(row, col, chessboard) {
            chessboard[row][col] = QUEEN;
            dfs(row + 1, size, chessboard, result);
            chessboard[row][col] = EMPTY;
        }
    }
}

fn is_valid(row: usize, col: usize, chessboard: &Vec2D<char>) -> bool {
    // 同列上方
    (0..row).all(|row| chessboard[row][col] != QUEEN)
        // 对角左上
        && (1..=row.min(col)).all(|step| chessboard[row - step][col - step] != QUEEN)
        // 对角右上
        && (1..=row.min(chessboard.len() - 1 - col))
            .all(|step| chessboard[row - step][col + step] != QUEEN)
}
// @lc code=end
