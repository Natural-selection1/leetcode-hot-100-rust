/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

use crate::Solution;
// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut not_zero_num_index = 0;
        for walking_index in 0..nums.len() {
            if nums[walking_index] != 0 {
                nums.swap(walking_index, not_zero_num_index);
                not_zero_num_index += 1;
            }
        }
    }
}
// @lc code=end
