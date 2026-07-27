/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut min_cost = vec![vec![0; col_count]; row_count];

        for i in 0..row_count {
            for j in 0..col_count {
                let cost = grid[i][j];
                match (i, j) {
                    (0, 0) => min_cost[i][j] = cost,                      // 起点
                    (0, _) => min_cost[i][j] = cost + min_cost[i][j - 1], // 第一行
                    (_, 0) => min_cost[i][j] = cost + min_cost[i - 1][j], // 第一列
                    (_, _) => min_cost[i][j] = cost + min_cost[i - 1][j].min(min_cost[i][j - 1]),
                }
            }
        }

        min_cost[row_count - 1][col_count - 1]
    }

    // 一维dp
    pub fn min_path_sum_(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid[0].len();
        let mut min_cost = vec![0; row_count];

        for (i, cost) in grid.iter().enumerate() {
            for j in 0..row_count {
                let cost = cost[j];
                match (i, j) {
                    (0, 0) => min_cost[j] = cost,                   // 起点
                    (0, _) => min_cost[j] = cost + min_cost[j - 1], // 第一行
                    (_, 0) => min_cost[j] += cost,                  // 第一列
                    (_, _) => min_cost[j] = cost + min_cost[j - 1].min(min_cost[j]),
                }
            }
        }

        min_cost[row_count - 1]
    }
}
// @lc code=end
