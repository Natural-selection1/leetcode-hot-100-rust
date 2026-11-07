/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 *
 * https://leetcode.cn/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (72.50%)
 * Likes:    1963
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 1.4M
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
 *
 * 说明：每次只能向下或者向右移动一步。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
 * 输出：7
 * 解释：因为路径 1→3→1→1→1 的总和最小。
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [[1,2,3],[4,5,6]]
 * 输出：12
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 200
 * 0 <= grid[i][j] <= 200
 *
 *
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }

    // 一维dp
    pub fn min_path_sum_(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

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

*/
