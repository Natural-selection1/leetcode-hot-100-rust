/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */

// @lc code=start
impl crate::Solution {
    pub fn num_squares(target_num: i32) -> i32 {
        let target_num = target_num as usize;
        let mut min_squares_sum_of = vec![i32::MAX / 2; target_num + 1];
        min_squares_sum_of[0] = 0;

        for walk_num in 1..=target_num.isqrt() {
            for i in walk_num.pow(2)..=target_num {
                min_squares_sum_of[i] =
                    min_squares_sum_of[i].min(min_squares_sum_of[i - walk_num.pow(2)] + 1);
            }
        }

        min_squares_sum_of[target_num]
    }
}
// @lc code=end
