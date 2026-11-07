/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 *
 * https://leetcode.cn/problems/perfect-squares/description/
 *
 * algorithms
 * Medium (68.38%)
 * Likes:    2324
 * Dislikes: 0
 * Total Accepted:    909.8K
 * Total Submissions: 1.3M
 * Testcase Example:  '12'
 *
 * 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。
 *
 * 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11
 * 不是。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 12
 * 输出：3
 * 解释：12 = 4 + 4 + 4
 *
 * 示例 2：
 *
 *
 * 输入：n = 13
 * 输出：2
 * 解释：13 = 4 + 9
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 10^4
 *
 *
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn num_squares_(target: i32) -> i32 {
        todo!()
    }

    // 一维dp
    pub fn num_squares(target: i32) -> i32 {
        /*
        dp数组含义:
        dp[j] 表示当数值为j时, 凑成它的最小完全平方数的个数

        递推公式:
        dp[j] = dp[j].min(dp[j - val[i]])

        初始化:
         */
        let target = target as usize;
        let mut squares = vec![];
        for i in 1..target {
            if i * i > target { break }
            squares.push(i*i);
        }

        let mut dp = vec![];
        for i in 0..target + 1{
            dp.push(i);
        }
        for square in squares {
            for j in square..target+1 {
                dp[j] = dp[j].min(dp[j - square] + 1);
            }
        }
        dp[target] as i32
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

// 二维dp
pub fn num_squares(target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![vec![i32::MAX; target + 1]; 101];

        dp[0][0] = 0;
        for i in 1..=((target as f64).sqrt() as usize) {
            for j in 0..=target {
                match j < (i * i) {
                    // 来自上一行同列
                    true => dp[i][j] = dp[i - 1][j],
                    // 来自上一行同列 或 当前行但列数减去i的平方
                    false => dp[i][j] = dp[i - 1][j].min(dp[i][j - (i * i)] + 1),
                }
            }
        }

        dp[(target as f64).sqrt() as usize][target]
    }

// 一维dp
pub fn num_squares_(target: i32) -> i32 {
        let target = target as usize;
        let mut min_squares_sum: Vec<i32> = (0..=target).map(|i| i as i32).collect();
        // 以 12 举例
        // 列标:               0  1   2   3  4   5   6   7   8  9   10  11  12
        // 初始化(1开始):      0  1   2   3  4   5   6   7   8  9   10  11  12
        // 2:                  0  1   2   3  1   2   3   4   2  3   4   5   3   // 只更新了4之后的列
        // 3:                  0  1   2   3  1   2   3   4   2  1   2   3   3   // 只更新了9之后的列
        //
        // 行标从 2, 3 ... target.isqrt()
        // 列标只更新从行标的平方开始到target
        for num in 2..=(target as f64).sqrt() as usize {
            for i in (num * num)..=target {
                min_squares_sum[i] = min_squares_sum[i].min(min_squares_sum[i - (num * num)] + 1);
            }
        }

        min_squares_sum[target]
    }
}
*/
