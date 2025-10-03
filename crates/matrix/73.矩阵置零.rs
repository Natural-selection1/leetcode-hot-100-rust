/*
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * [73] 矩阵置零
 */

// @lc code=start
impl crate::Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row_len = matrix[0].len();
        let mut zeros_mark: Vec<(usize, usize)> = vec![];

        // 记录所有0的位置
        for (i, row) in matrix.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                (num == &0).then(|| zeros_mark.push((i, j)));
            }
        }

        for (i, j) in zeros_mark {
            matrix.iter_mut().for_each(|row| row[j] = 0);
            (0..row_len).for_each(|j| matrix[i][j] = 0);
        }
    }
}
// @lc code=end
