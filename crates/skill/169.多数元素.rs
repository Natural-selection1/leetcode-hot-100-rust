/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl crate::Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}
// @lc code=end
