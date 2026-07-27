/*
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * [73] 矩阵置零
 */

// @lc code=start
impl crate::Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zeros_coordinate: Vec<(usize, usize)> = vec![];

        // 记录所有0的位置
        for (i, row) in matrix.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if num == &0 {
                    zeros_coordinate.push((i, j));
                }
            }
        }

        for (row, column) in zeros_coordinate {
            matrix.iter_mut().for_each(|row| row[column] = 0);
            (0..matrix[0].len()).for_each(|column| matrix[row][column] = 0);
        }
    }
}
// @lc code=end
