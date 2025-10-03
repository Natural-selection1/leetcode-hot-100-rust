/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn num_squares_(target_num: i32) -> i32 {
        let target_num = target_num as usize;
        let mut dp = vec![vec![i32::MAX; target_num + 1]; 101];

        dp[0][0] = 0;
        for i in 1..=target_num.isqrt() {
            for j in 0..=target_num {
                match j < i.pow(2) {
                    // 来自上一行同列
                    true => dp[i][j] = dp[i - 1][j],
                    // 来自上一行同列 或 当前行但列数减去i的平方
                    false => dp[i][j] = dp[i - 1][j].min(dp[i][j - i.pow(2)] + 1),
                }
            }
        }

        dp[target_num.isqrt()][target_num]
    }

    // 一维dp
    pub fn num_squares(target_num: i32) -> i32 {
        let target_num = target_num as usize;
        let mut min_squares_sum: Vec<i32> = (0..=target_num).map(|i| i as i32).collect();
        // 以 12 举例
        // 列标:               0  1   2   3  4   5   6   7   8  9   10  11  12
        // 初始化(1开始):      0  1   2   3  4   5   6   7   8  9   10  11  12
        // 2:                  0  1   2   3  1   2   3   4   2  3   4   5   3   // 只更新了4之后的列
        // 3:                  0  1   2   3  1   2   3   4   2  1   2   3   3   // 只更新了9之后的列
        //
        // 行标从 2, 3 ... target_num.isqrt()
        // 列标只更新从行标的平方开始到target_num
        for num in 2..=target_num.isqrt() {
            for i in num.pow(2)..=target_num {
                min_squares_sum[i] = min_squares_sum[i].min(min_squares_sum[i - num.pow(2)] + 1);
            }
        }

        min_squares_sum[target_num]
    }
}
// @lc code=end
