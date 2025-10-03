/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl crate::Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // * 标准库写法
        // nums.partition_point(|&x| x < target) as _

        // * 手写
        // lowerBound 返回最小的满足 nums[i] >= target 的 i
        // 如果数组为空，或者所有数都 < target，则返回 nums.length
        // 要求 nums 是非递减的，即 nums[i] <= nums[i + 1]
        lower_bound(&nums, target)
    }
}

// 左闭右开区间写法
fn lower_bound(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len(); // 左闭右开区间 [left, right)
    // 区间不为空
    while left < right {
        // 循环不变量：
        // nums[left-1] < target
        // nums[right] >= target
        let mid = left + (right - left) / 2;
        match nums[mid] < target {
            true => left = mid + 1,
            false => right = mid, // 范围缩小到 [left, mid)
        }
    }
    left as _
}
// @lc code=end
