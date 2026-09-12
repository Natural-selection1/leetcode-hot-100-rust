/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 *
 * https://leetcode.cn/problems/min-cost-climbing-stairs/description/
 *
 * algorithms
 * Easy (68.11%)
 * Likes:    1806
 * Dislikes: 0
 * Total Accepted:    664.3K
 * Total Submissions: 976.3K
 * Testcase Example:  '[10,15,20]'
 *
 * 给你一个整数数组 cost ，其中 cost[i] 是从楼梯第 i 个台阶向上爬需要支付的费用。
 *
 * 一旦你支付此费用，即可选择向上爬 一个 或者 两个 台阶。
 *
 * 你可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯。
 *
 * 请你计算并返回达到楼梯顶部的 最低 花费，即最后一步之后的那个位置（下标 cost.length）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：cost = [10,15,20]
 * 输出：15
 * 解释：你将从下标为 1 的台阶开始。
 * - 支付 15 ，向上爬两个台阶，到达楼梯顶部。
 * 总花费为 15 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：cost = [1,100,1,1,1,100,1,1,100,1]
 * 输出：6
 * 解释：你将从下标为 0 的台阶开始。
 * - 支付 1 ，向上爬两个台阶，到达下标为 2 的台阶。
 * - 支付 1 ，向上爬两个台阶，到达下标为 4 的台阶。
 * - 支付 1 ，向上爬两个台阶，到达下标为 6 的台阶。
 * - 支付 1 ，向上爬一个台阶，到达下标为 7 的台阶。
 * - 支付 1 ，向上爬两个台阶，到达下标为 9 的台阶。
 * - 支付 1 ，向上爬一个台阶，到达楼梯顶部。
 * 总花费为 6 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= cost.length <= 1000
 * 0 <= cost[i] <= 999
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.push(0);
        // dp数组下标及其值的含义:
        // 爬到第 n 阶时, 最小花费为 dp[n-1]

        // 递推公式:
        // dp[n] = (dp[n-1] + cost[n-1]).min(dp[n-2] + cost[n-2])

        // 初始化:
        let mut dp = vec![0; cost.len()];
        dp[0] = 0;
        dp[1] = 0;

        for i in 2..cost.len() {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2])
        }

        *dp.last().unwrap()
    }
}
// @lc code=end
