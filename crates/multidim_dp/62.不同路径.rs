/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn unique_paths(length: i32, width: i32) -> i32 {
        if length.min(width) == 1 {
            return 1;
        }

        let length = length as usize;
        let width = width as usize;
        let mut dp = vec![vec![1; width]; length];

        for i in 1..length {
            for j in 1..width {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[length - 1][width - 1]
    }

    // 一维dp
    pub fn unique_paths_(length: i32, width: i32) -> i32 {
        if length.min(width) == 1 {
            return 1;
        }

        let length = length as usize;
        let width = width as usize;
        let mut dp = vec![1; length];

        (1..width).for_each(|_| (1..length).for_each(|i| dp[i] += dp[i - 1]));

        dp[length - 1]
    }
}
// @lc code=end
