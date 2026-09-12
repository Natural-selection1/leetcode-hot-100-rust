/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 * [343] 整数拆分
 *
 * https://leetcode.cn/problems/integer-break/description/
 *
 * algorithms
 * Medium (65.03%)
 * Likes:    1561
 * Dislikes: 0
 * Total Accepted:    457.6K
 * Total Submissions: 703K
 * Testcase Example:  '2'
 *
 * 给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。
 *
 * 返回 你可以获得的最大乘积 。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: n = 2
 * 输出: 1
 * 解释: 2 = 1 + 1, 1 × 1 = 1。
 *
 * 示例 2:
 *
 *
 * 输入: n = 10
 * 输出: 36
 * 解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
 *
 *
 *
 * 提示:
 *
 *
 * 2 <= n <= 58
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        // dp[i][j]: 用不超过 i 的数拆分 j（至少两个数）的最大乘积
        let length = n + 1;
        let mut dp = vec![vec![0i32; length]; length];

        // 初始化：只用 1 拆分 j
        for j in 2..length {
            dp[1][j] = 1;
        }

        for i in 2..length {
            for j in 2..length {
                // 不用 i
                let mut best = dp[i - 1][j];

                // 用 i，剩下的 j-i 不拆
                if j > i {
                    best = best.max((i * (j - i)) as i32);
                }

                // 用 i，剩下的 j-i 继续拆
                if j > i && dp[i][j - i] > 0 {
                    best = best.max(i as i32 * dp[i][j - i]);
                }

                dp[i][j] = best;
            }
        }

        dp[n][n]
    }
}
// @lc code=end
