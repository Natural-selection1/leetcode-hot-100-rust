/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl crate::Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let mut answer = 0;

        nums.iter().enumerate().take(nums.len() - 1).fold(
            (0, 0),
            |(current_round_reachable_index, next_round_reachable_index),
             (current_index, &jump_distance)| {
                let next_round_reachable_index =
                    next_round_reachable_index.max(current_index + jump_distance);

                match current_index != current_round_reachable_index {
                    true => (current_round_reachable_index, next_round_reachable_index),
                    false => {
                        answer += 1;
                        (next_round_reachable_index, next_round_reachable_index)
                    }
                }
            },
        );

        answer
    }
}
// @lc code=end
