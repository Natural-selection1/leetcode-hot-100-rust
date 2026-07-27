/*
 * @lc app=leetcode.cn id=240 lang=rust
 *
 * [240] 搜索二维矩阵 II
 */

// @lc code=start
use std::cmp::Ordering;

impl crate::Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let column_count = matrix[0].len();
        let row_count = matrix.len();
        // 从右上角开始向左下搜索, 因为它有以下两个性质:
        // 1. 是当前行的最大值:
        //    如果它比目标值小 => 整行都比目标值小, 向下一行搜索
        // 2. 是当前列的最小值:
        //    如果它比目标值大 => 整列都比目标值大, 向左一列搜索
        let (mut row, mut column) = (0, column_count - 1);
        while row < row_count && column < column_count {
            match matrix[row][column].cmp(&target) {
                Ordering::Greater => column -= 1,
                Ordering::Less => row += 1,
                Ordering::Equal => return true,
            }
        }

        false
    }
}
// @lc code=end
