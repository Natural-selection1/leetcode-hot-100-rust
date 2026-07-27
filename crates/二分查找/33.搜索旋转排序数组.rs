/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */

// @lc code=start
use std::ops::{Add, Div};

impl crate::Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let last = nums.last();
        let target_in_first_segment = Some(&target) > last;
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = midpoint(left, right);
            let mid_in_first_segment = Some(&nums[mid]) > last;

            match (target_in_first_segment, mid_in_first_segment) {
                // target 和 mid 在不同段
                (true, false) => right = mid,
                (false, true) => left = mid + 1,
                // target 和 mid 在同一段，正常二分
                _ => match nums[mid] >= target {
                    true => right = mid,
                    false => left = mid + 1,
                },
            }
        }

        if nums[left] == target {
            left as _
        } else {
            -1
        }
    }
}

fn midpoint<T: Add>(a: T, b: T) -> T
where
    <T as Add>::Output: Div<usize, Output = T>,
{
    (a + b) / 2
}

// @lc code=end
