/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

// @lc code=start
impl crate::Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let col_len = grid[0].len();
        let mut min_cost_when = vec![0; col_len];

        for (row_index, cost_of) in grid.iter().enumerate() {
            for col_index in 0..col_len {
                let current_cost = cost_of[col_index];

                match (row_index, col_index) {
                    (0, 0) => min_cost_when[col_index] = current_cost,
                    (_, 0) => min_cost_when[col_index] += current_cost,
                    (0, _) => {
                        min_cost_when[col_index] = current_cost + min_cost_when[col_index - 1]
                    }
                    (_, _) => {
                        min_cost_when[col_index] = current_cost
                            + min_cost_when[col_index - 1].min(min_cost_when[col_index])
                    }
                }
            }
        }

        min_cost_when[col_len - 1]
    }
}
// @lc code=end
