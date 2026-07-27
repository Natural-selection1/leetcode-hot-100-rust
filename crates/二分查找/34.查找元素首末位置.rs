/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
impl crate::Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 两次二分
        let start = nums.partition_point(|&x| x < target) as i32;
        let end = nums.partition_point(|&x| x <= target) as i32;

        match start == end {
            true => vec![-1, -1],
            false => vec![start, end - 1],
        }
    }
}
// @lc code=end
