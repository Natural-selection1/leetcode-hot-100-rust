/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start
impl crate::Solution {
    pub fn rob(values: Vec<i32>) -> i32 {
        values
            .iter()
            .skip(1)
            .fold((0, values[0]), |(n1, n2), cur_room_val| {
                (n2, (n1 + cur_room_val).max(n2))
            })
            .1
    }
}
// @lc code=end
