/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn longest_common_subsequence_(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();
        let mut lcs_when = vec![vec![0; len2 + 1]; len1 + 1];

        for (i, char1) in text1.as_bytes().iter().enumerate() {
            for (j, char2) in text2.as_bytes().iter().enumerate() {
                match char1 == char2 {
                    true => lcs_when[i + 1][j + 1] = lcs_when[i][j] + 1,
                    false => lcs_when[i + 1][j + 1] = lcs_when[i][j + 1].max(lcs_when[i + 1][j]),
                }
            }
        }

        lcs_when[len1][len2]
    }

    // 一维dp
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len2 = text2.len();
        let mut lcs_when = vec![0; len2 + 1];

        for char1 in text1.as_bytes() {
            let mut prev = 0; // 代表二维中的 dp[i][j] 即左上角的值

            for (col, char2) in text2.as_bytes().iter().enumerate() {
                let cur = lcs_when[col + 1]; // 记录未更新前的值, 留作更新 prev

                match char1 == char2 {
                    true => lcs_when[col + 1] = prev + 1,
                    false => lcs_when[col + 1] = lcs_when[col + 1].max(lcs_when[col]),
                }
                prev = cur;
            }
        }

        lcs_when[len2]
    }
}
// @lc code=end
