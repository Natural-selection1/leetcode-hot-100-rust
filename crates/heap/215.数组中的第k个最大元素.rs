/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
use std::collections::BinaryHeap;

impl crate::Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 耗时O(n)的原地转换
        let mut heap = BinaryHeap::from(nums);
        (0..(k - 1)).for_each(|_| _ = heap.pop());
        #[allow(clippy::unwrap_used, reason = "测试保证k符合要求")]
        *heap.peek().unwrap()
    }
}
// @lc code=end
