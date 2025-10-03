/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 */

// @lc code=start
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右 下 左 上

impl crate::Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let col_len = matrix.len();
        let row_len = matrix[0].len();

        // 每次步长：[row_len, col_len-1, row_len-1, col_len-2, ...]
        let steps: Vec<usize> = (0..row_len.min(col_len))
            .flat_map(|turns_time| [row_len - turns_time, col_len - 1 - turns_time])
            .collect();

        let mut spiral_matrix = Vec::with_capacity(col_len * row_len);
        let (mut i, mut j) = (0, -1);
        for (turns_time, &current_step) in steps.iter().enumerate() {
            let (dx, dy) = DIRECTIONS[turns_time % 4];
            for _ in 0..current_step {
                i += dx;
                j += dy;
                spiral_matrix.push(matrix[i as usize][j as usize]);
            }
        }

        spiral_matrix
    }
}
// @lc code=end
