/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 */

// @lc code=start
impl crate::Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = nums.last();
        // 左闭右开区间 [0, n-1)
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left.midpoint(right);

            match Some(&nums[mid]) < last {
                true => right = mid,
                false => left = mid + 1,
            }
        }

        nums[left]
    }
}

// @lc code=end
