/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 */

// @lc code=start
use std::ops::{Add, Div};

impl crate::Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = *nums.last().expect("nums is empty");
        // 左闭右开区间 [0, n-1)
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = midpoint(left, right);

            match nums[mid] < last {
                true => right = mid,
                false => left = mid + 1,
            }
        }

        nums[left]
    }
}

fn midpoint<T: Add>(left: T, right: T) -> T
where
    <T as Add>::Output: Div<usize, Output = T>,
{
    (left + right) / 2
}
// @lc code=end
