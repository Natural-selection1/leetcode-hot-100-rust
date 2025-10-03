/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl crate::Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = 0;
        let mut prefix_min_sum = 0;
        let mut max_sub_array_sum = i32::MIN;

        for num in nums {
            prefix_sum += num;
            let differen = prefix_sum - prefix_min_sum;

            max_sub_array_sum = max_sub_array_sum.max(differen);
            prefix_min_sum = prefix_min_sum.min(prefix_sum);
        }

        max_sub_array_sum
    }
}
// @lc code=end
