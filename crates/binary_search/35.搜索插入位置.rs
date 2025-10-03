/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
use std::cmp::Ordering;

impl crate::Solution {
    // 标准库
    pub fn search_insert_(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&num| num < target) as _
    }

    // 手写
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // 左闭右开区间 [left, right)
        let mut left = 0;
        let mut right = nums.len();

        // 循环不变量：
        // nums[left-1] < target
        // nums[right] >= target
        while left < right {
            let mid = left.midpoint(right);

            match nums[mid].cmp(&target) {
                Ordering::Greater => right = mid,
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid as _,
            }
        }

        left as _
    }
}
// @lc code=end
