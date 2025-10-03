/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
impl crate::Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        nums.into_iter()
            .enumerate()
            .try_fold(0, |max_reachable_index, (current_index, jump_distance)| {
                (current_index <= max_reachable_index)
                    .then_some(max_reachable_index.max(current_index + jump_distance as usize))
            })
            .is_some()
    }
}
// @lc code=end
