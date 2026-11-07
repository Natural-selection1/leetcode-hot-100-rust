/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 *
 * https://leetcode.cn/problems/unique-binary-search-trees/description/
 *
 * algorithms
 * Medium (71.46%)
 * Likes:    2753
 * Dislikes: 0
 * Total Accepted:    573.9K
 * Total Submissions: 803.7K
 * Testcase Example:  '3'
 *
 * 给你一个整数 n ，求恰由 n 个节点组成且节点值从 1 到 n 互不相同的 二叉搜索树 有多少种？返回满足题意的二叉搜索树的种数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：5
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        // dp数组含义:
        // dp[i] 指由 i 个节点构成的二叉搜索树的个数

        // 初始化:
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;

        for node_count in 3..=n {
            for head in 1..=node_count {
                let left = head - 1;
                let right = node_count - head;

                dp[i] += dp[left] * dp[right]
            }
        }

        dp[n]
    }
}
// @lc code=end
