/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start
use std::cmp::Ordering;

impl crate::Solution {
    // 暴力二分
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix
            .into_iter()
            .find(|row| row.last() >= Some(&target))
            .is_some_and(|row| row.binary_search(&target).is_ok())
    }

    // 利用单调性
    pub fn search_matrix_(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i = 0;
        let mut j_ = Some(matrix[0].len() - 1);

        while let Some(j) = j_
            && let Some(num) = matrix.get(i).and_then(|row| row.get(j))
        {
            match num.cmp(&target) {
                Ordering::Greater => j_ = j.checked_sub(1),
                Ordering::Less => i += 1,
                Ordering::Equal => return true,
            }
        }

        false
    }
}

// @lc code=end
