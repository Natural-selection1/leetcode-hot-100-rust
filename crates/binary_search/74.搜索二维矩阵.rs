/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start
use std::cmp::Ordering;

impl crate::Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // matrix
        //     .into_iter()
        //     .flatten()
        //     .collect::<Vec<i32>>()
        //     .binary_search(&target)
        //     .is_ok()

        let mut i = 0;
        let mut j = matrix[0].len() as i32 - 1;
        while i < matrix.len() && j >= 0 {
            // 还有剩余元素
            match matrix[i][j as usize].cmp(&target) {
                Ordering::Greater => j -= 1,    // 这一列剩余元素全部大于 target，排除
                Ordering::Less => i += 1,       // 这一行剩余元素全部小于 target，排除
                Ordering::Equal => return true, // 找到 target
            }
        }

        false
    }
}

// @lc code=end
