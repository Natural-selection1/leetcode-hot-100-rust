/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl crate::Solution {
    pub fn unique_paths(length: i32, width: i32) -> i32 {
        if length.min(width) == 1 {
            return 1;
        }

        let length = length as usize;
        let width = width as usize;
        let mut dp = vec![1; length];

        for _ in 1..width {
            for i in 1..length {
                dp[i] += dp[i - 1];
            }
        }

        dp[length - 1]
    }
}
// @lc code=end
