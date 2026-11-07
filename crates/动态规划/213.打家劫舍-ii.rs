/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 *
 * https://leetcode.cn/problems/house-robber-ii/description/
 *
 * algorithms
 * Medium (46.08%)
 * Likes:    1804
 * Dislikes: 0
 * Total Accepted:    554.2K
 * Total Submissions: 1.2M
 * Testcase Example:  '[2,3,2]'
 *
 * 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈
 * ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
 *
 * 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,3,2]
 * 输出：3
 * 解释：你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3,1]
 * 输出：4
 * 解释：你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
 * 偷窃到的最高金额 = 1 + 3 = 4 。
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,2,3]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        /*
        dp数组含义:
        dp[i] 表示偷到第 i 个房间时的最大价值

        递推公式:
        dp[i] = dp[i - 1].max(dp[i - 2] + values[i]);

        分组:
         */

         if nums.len() == 1 {
            return nums[0];
         }

        rob_inner(&nums[0..nums.len() - 1])
            .max(rob_inner(&nums[1..nums.len()]))
    }
}

fn rob_inner(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut dp = vec![0; nums.len()+ 1];
    dp[0] = 0;
    dp[1] = nums[0];

    for i in 2..nums.len() + 1 {
        dp[i] = dp[i -1].max(dp[i-2] + nums[i-1]);
    }
    dp[nums.len()]
}
// @lc code=end
