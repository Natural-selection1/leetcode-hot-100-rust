/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 *
 * https://leetcode.cn/problems/edit-distance/description/
 *
 * algorithms
 * Medium (64.07%)
 * Likes:    3970
 * Dislikes: 0
 * Total Accepted:    932.6K
 * Total Submissions: 1.5M
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。
 *
 * 你可以对一个单词进行如下三种操作：
 *
 *
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：word1 = "horse", word2 = "ros"
 * 输出：3
 * 解释：
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 *
 *
 * 示例 2：
 *
 *
 * 输入：word1 = "intention", word2 = "execution"
 * 输出：5
 * 解释：
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= word1.length, word2.length <= 500
 * word1 和 word2 由小写英文字母组成
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        /*
        dp数组含义:
        取长度为i且结尾的 word1 切片, 与取长度为j且结尾的 word2 切片
        构成的最短编辑距离为 dp[i][j]

        递推公式:

        初始化:
         */

        // [0] 行/列表示空串
        let mut dp = vec![vec![usize::MAX / 2; word2.len() + 1]; word1.len() + 1];

        for j in 0..word2.len() + 1 {
            dp[0][j] = j
        }
        for i in 0..word1.len() + 1 {
            dp[i][0] = i
        }

        for i in 1..word1.len() + 1 {
            for j in 1..word2.len() + 1 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1
                }
            }
        }

        dp[word1.len()][word2.len()] as i32
    }
}
// @lc code=end
