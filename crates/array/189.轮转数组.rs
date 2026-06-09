/*
 * @lc app=leetcode.cn id=189 lang=rust
 *
 * [189] 轮转数组
 */

// @lc code=start
impl crate::Solution {
    pub fn rotate_(nums: &mut [i32], rotate_times: i32) {
        let rotate_times = rotate_times as usize % nums.len();

        nums.reverse();
        nums[rotate_times..].reverse();
        nums[..rotate_times].reverse();
    }

    pub fn rotate(nums: &mut [i32], rotate_times: i32) {
        nums.rotate_right(rotate_times as usize % nums.len());
    }
}
// @lc code=end
