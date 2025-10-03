/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */

// @lc code=start
impl crate::Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let matrix_len = matrix.len();

        //  前面的所有的行都处理过后, 最后一行就已经整好了
        for row in 0..(matrix_len - 1) {
            // 不含对角线的下三角形
            for column in (row + 1)..matrix_len {
                (matrix[row][column], matrix[column][row]) =
                    (matrix[column][row], matrix[row][column]);
            }
        }
        matrix.iter_mut().for_each(|row| row.reverse());
    }
}
// @lc code=end
