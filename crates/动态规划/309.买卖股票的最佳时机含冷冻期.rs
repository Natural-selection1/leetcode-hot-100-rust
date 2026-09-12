/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 买卖股票的最佳时机含冷冻期
 *
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
 *
 * algorithms
 * Medium (65.51%)
 * Likes:    1935
 * Dislikes: 0
 * Total Accepted:    435.5K
 * Total Submissions: 664.1K
 * Testcase Example:  '[1,2,3,0,2]'
 *
 * 给定一个整数数组prices，其中第  prices[i] 表示第 i 天的股票价格 。​
 *
 * 设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:
 *
 *
 * 卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。
 *
 *
 * 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: prices = [1,2,3,0,2]
 * 输出: 3
 * 解释: 对应的交易状态为: [买入, 卖出, 冷冻期, 买入, 卖出]
 *
 * 示例 2:
 *
 *
 * 输入: prices = [1]
 * 输出: 0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= prices.length <= 5000
 * 0 <= prices[i] <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /*
        dp数组含义:
        dp[0][j]: 在第j天保持无操作
        dp[1][j]: 在第j天持有股票
        dp[2][j]: 在第j天不持有股票

        递推公式:
        dp[1][j] = max(
            dp[1][j-1], // 保持持有
            dp[0][j-1] - prices[j], // 从无操作变为买入
            dp[2][j - k] - prices[j], // 从上次卖出后变为买入
        )

        dp[2][j] = max(
            dp[2][j - 1], // 保持不持有
            dp[1][j - 1] + prices[j], // 卖出
        )

        初始化:
        dp[0][j] = 0
        dp[1][0] = -prices[0]
        dp[2][0] = 0
         */

        let mut dp = vec![vec![0; prices.len()]; 3];
        dp[1][0] = -prices[0];

        use std::cmp::Ord;
        for j in 1..prices.len() {
            dp[1][j] = dp[1][j - 1].max(dp[0][j - 1] - prices[j]);
            if j >= 2 {
                dp[1][j] = dp[1][j].max(dp[2][j - 2] - prices[j])
            };

            dp[2][j] = Ord::max(
                dp[2][j - 1],             // 保持不持有
                dp[1][j - 1] + prices[j], // 卖出
            );
        }

        dp[0][prices.len() - 1]
            .max(dp[1][prices.len() - 1])
            .max(dp[2][prices.len() - 1])
    }
}
// @lc code=end
