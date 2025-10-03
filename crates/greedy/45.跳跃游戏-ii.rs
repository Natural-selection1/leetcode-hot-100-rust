/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl crate::Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut answer = 0;

        (0..nums.len() - 1)
            .zip(nums.iter())
            .map(|(x, y)| (x as i32, y))
            .fold(
                (0, 0),
                |(current_reachable_i, next_reachable_i), (walk_index, &jump_distance)| {
                    let next_reachable_i = next_reachable_i.max(walk_index + jump_distance);
                    match current_reachable_i == walk_index {
                        true => {
                            answer += 1;
                            (next_reachable_i, next_reachable_i)
                        }
                        false => (current_reachable_i, next_reachable_i),
                    }
                },
            );

        answer
    }
}
// @lc code=end
