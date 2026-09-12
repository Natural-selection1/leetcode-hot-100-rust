/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 *
 * https://leetcode.cn/problems/house-robber/description/
 *
 * algorithms
 * Medium (56.35%)
 * Likes:    3568
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 3M
 * Testcase Example:  '[1,2,3,1]'
 *
 *
 * 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
 *
 * 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：[1,2,3,1]
 * 输出：4
 * 解释：偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
 * 偷窃到的最高金额 = 1 + 3 = 4 。
 *
 * 示例 2：
 *
 *
 * 输入：[2,7,9,3,1]
 * 输出：12
 * 解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
 * 偷窃到的最高金额 = 2 + 9 + 1 = 12 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn rob(values: Vec<i32>) -> i32 {
        /*
        dp数组含义:
        偷到第i个房间时的最大价值为 dp[i]

        递推公式:
        dp[i] = dp[i-1].max(dp[i-2] + value[i])

        初始化:
        0下标: dp[0] = value[0]
        非0下标: dp[i] = 0
         */

        let mut dp = vec![0; values.len() + 1]; // 多加一个无价值房间
        dp[0] = 0;
        dp[1] = values[0];

        for i in 2..(values.len() + 1) {
            dp[i] = dp[i - 1].max(dp[i - 2] + values[i - 1])
        }

        dp[values.len()]
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn rob(values: Vec<i32>) -> i32 {
        values
            .iter()
            .skip(1)
            .fold(
                (0, values[0]),
                // 状态转移:
                //     dp[i]   = max(dp[i-1], dp[i-2] + val)
                |(n1, n2), val| {
                    // dp[i+1] = max(dp[i]  , dp[i-1] + val)
                    (n2, n2.max(n1 + val))
                },
            )
            .1
    }
}
*/
