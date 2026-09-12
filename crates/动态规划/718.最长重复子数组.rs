/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 *
 * https://leetcode.cn/problems/maximum-length-of-repeated-subarray/description/
 *
 * algorithms
 * Medium (56.99%)
 * Likes:    1255
 * Dislikes: 0
 * Total Accepted:    355K
 * Total Submissions: 622.7K
 * Testcase Example:  '[1,2,3,2,1]\n[3,2,1,4,7]'
 *
 * 给两个整数数组 nums1 和 nums2 ，返回 两个数组中 公共的 、长度最长的子数组的长度 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
 * 输出：3
 * 解释：长度最长的公共子数组是 [3,2,1] 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
 * 输出：5
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums1.length, nums2.length <= 1000
 * 0 <= nums1[i], nums2[i] <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        /*
        dp数组含义:
        dp[i][j]: nums1[0..i] 为结尾的 与 nums2[0..j] 为结尾的,
                构成的最长重复子数组的长度为 dp[i][j]

        递推公式:
        if nums1[i] == nums2[j] {
            dp[i][j] = dp[i- 1][j -1] + 1
        }
         */

        // dp[0][j] 表示nums1的空切片到nums2
        // dp[i][0] 表示nums2的空切片到nums1
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut max = 0;

        for i in 1..nums1.len() + 1 {
            for j in 1..nums2.len() + 1 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    max = max.max(dp[i][j])
                }
            }
        }

        max
    }
}
// @lc code=end
