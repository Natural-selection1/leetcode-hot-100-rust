/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 *
 * https://leetcode.cn/problems/longest-common-subsequence/description/
 *
 * algorithms
 * Medium (67.54%)
 * Likes:    1952
 * Dislikes: 0
 * Total Accepted:    904.1K
 * Total Submissions: 1.3M
 * Testcase Example:  '"abcde"\n"ace"'
 *
 * 给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
 *
 * 一个字符串的 子序列
 * 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
 *
 *
 * 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
 *
 *
 * 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：text1 = "abcde", text2 = "ace"
 * 输出：3
 * 解释：最长公共子序列是 "ace" ，它的长度为 3 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：text1 = "abc", text2 = "abc"
 * 输出：3
 * 解释：最长公共子序列是 "abc" ，它的长度为 3 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：text1 = "abc", text2 = "def"
 * 输出：0
 * 解释：两个字符串没有公共子序列，返回 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * text1 和 text2 仅由小写英文字符组成。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        /*
        dp数组含义:
        以 i 为长度且结尾的 text1, 与 以 j 为长度且结尾的 text2,
        构成的最长公共子序列的长度为dp[i][j]

        递推公式:

         */

        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let mut max = 0;

        for i in 1..text1.len() + 1 {
            for j in 1..text2.len() + 1 {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
                }
                max = dp[i][j].max(max)
            }
        }

        max
    }
}
// @lc code=end
