/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl crate::Solution {
    pub fn climb_stairs(times: i32) -> i32 {
        (1..=times).fold((0, 1), |(n1, n2), _| (n2, n1 + n2)).1
    }
}
// @lc code=end
