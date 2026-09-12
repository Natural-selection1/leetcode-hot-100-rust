/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 *
 * https://leetcode.cn/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (42.52%)
 * Likes:    1517
 * Dislikes: 0
 * Total Accepted:    728.1K
 * Total Submissions: 1.7M
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * 给定一个 m x n 的整数数组 grid。一个机器人初始位于 左上角（即 grid[0][0]）。机器人尝试移动到 右下角（即 grid[m -
 * 1][n - 1]）。机器人每次只能向下或者向右移动一步。
 *
 * 网格中的障碍物和空位置分别用 1 和 0 来表示。机器人的移动路径中不能包含 任何 有障碍物的方格。
 *
 * 返回机器人能够到达右下角的不同路径数量。
 *
 * 测试用例保证答案小于等于 2 * 10^9。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * 输出：2
 * 解释：3x3 网格的正中间有一个障碍物。
 * 从左上角到右下角一共有 2 条不同的路径：
 * 1. 向右 -> 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右 -> 向右
 *
 *
 * 示例 2：
 *
 *
 * 输入：obstacleGrid = [[0,1],[0,0]]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == obstacleGrid.length
 * n == obstacleGrid[i].length
 * 1 <= m, n <= 100
 * obstacleGrid[i][j] 为 0 或 1
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let length = obstacle_grid[0].len();
        let width = obstacle_grid.len();

        if length == 1 || width == 1 {
            for row in &obstacle_grid {
                for num in row {
                    if *num == 1 {
                        return 0;
                    }
                }
            }

            return 1;
        }
        // dp数组含义:
        // dp[i][j] 为 到达第i行第j列的路径数

        // 递推公式:
        // if obstacle_grid[i][j] == 1 { continue; }
        //   else { dp[i][j] = dp[i-1][j] + dp[i][j-1] }

        // 初始化:
        let mut dp = vec![vec![0; length]; width];
        for j in 0..length {
            match obstacle_grid[0][j] == 1 {
                true => break,
                false => dp[0][j] = 1,
            }
        }
        for i in 0..width {
            match obstacle_grid[i][0] == 1 {
                true => break,
                false => dp[i][0] = 1,
            }
        }

        // 遍历顺序: 无特殊
        for i in 1..width {
            for j in 1..length {
                match obstacle_grid[i][j] == 1 {
                    true => continue,
                    false => dp[i][j] = dp[i][j - 1] + dp[i - 1][j],
                }
            }
        }

        dp[width - 1][length - 1]
    }
}
// @lc code=end
