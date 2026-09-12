/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 *
 * https://leetcode.cn/problems/target-sum/description/
 *
 * algorithms
 * Medium (48.81%)
 * Likes:    2331
 * Dislikes: 0
 * Total Accepted:    670.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,1,1,1,1]\n3'
 *
 * 给你一个非负整数数组 nums 和一个整数 target 。
 *
 * 向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：
 *
 *
 * 例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
 *
 *
 * 返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,1,1,1,1], target = 3
 * 输出：5
 * 解释：一共有 5 种方法让最终目标和为 3 。
 * -1 + 1 + 1 + 1 + 1 = 3
 * +1 - 1 + 1 + 1 + 1 = 3
 * +1 + 1 - 1 + 1 + 1 = 3
 * +1 + 1 + 1 - 1 + 1 = 3
 * +1 + 1 + 1 + 1 - 1 = 3
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1], target = 1
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 20
 * 0 <= nums[i] <= 1000
 * 0 <= sum(nums[i]) <= 1000
 * -1000 <= target <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();

        if sum < target.abs() || (sum + target) % 2 != 0 {
            return 0;
        }

        let nums: Vec<usize> = nums.into_iter().map(|n| n as usize).collect();
        let volume = ((sum + target) / 2) as usize;
        let mut dp = vec![0_i32; volume + 1];
        dp[0] = 1;

        // 一维DP处理0-1背包一定要先遍历物品, 再遍历容量
        for num in nums {
            for j in (num..=volume).rev() {
                dp[j] += dp[j - num];
            }
        }

        dp[volume]
    }
}
// @lc code=end
