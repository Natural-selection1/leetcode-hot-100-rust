/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl crate::Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut answer = vec![vec![]; num_rows];

        for i in 0..num_rows {
            answer[i].resize(i + 1, 1);

            for j in 1..i {
                answer[i][j] = answer[i - 1][j - 1] + answer[i - 1][j];
            }
        }

        answer
    }
}

// @lc code=end
