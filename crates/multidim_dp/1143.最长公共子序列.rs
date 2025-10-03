/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
impl crate::Solution {
    pub fn longest_common_subsequence_(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for (i, char1) in text1.as_bytes().iter().enumerate() {
            for (j, char2) in text2.as_bytes().iter().enumerate() {
                match char1 == char2 {
                    true => dp[i + 1][j + 1] = dp[i][j] + 1,
                    false => dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]),
                }
            }
        }

        dp[len1][len2]
    }

    // one_dimensional_longest_common_subsequence
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len2 = text2.len();
        let mut dp = vec![0; len2 + 1];

        for char1 in text1.as_bytes() {
            let mut prev = 0; // 代表二维中的 dp[i][j] 即左上角的值

            for (col, char2) in text2.as_bytes().iter().enumerate() {
                let cur = dp[col + 1]; // 记录未更新前的值, 留作更新 prev
                match char1 == char2 {
                    true => dp[col + 1] = prev + 1,
                    false => dp[col + 1] = dp[col + 1].max(dp[col]),
                }
                prev = cur;
            }
        }

        dp[len2]
    }
}
// @lc code=end
