/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

// @lc code=start
impl crate::Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut not_zero_num_index = 0;

        for cursor in 0..nums.len() {
            if nums[cursor] != 0 {
                nums.swap(cursor, not_zero_num_index);
                not_zero_num_index += 1;
            }
        }
    }
}
// @lc code=end
