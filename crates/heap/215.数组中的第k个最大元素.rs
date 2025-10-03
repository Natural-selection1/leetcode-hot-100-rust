/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
use std::collections::BinaryHeap;

impl crate::Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums);
        (0..(k - 1)).for_each(|_| {
            heap.pop();
        });
        *heap.peek().unwrap()

        // let k = k as usize;
        // let mut heap = BinaryHeap::with_capacity(k);
        // for &num in &nums[..k] {
        //     heap.push(Reverse(num));
        // }
        // for &num in &nums[k..] {
        //     if num > heap.peek().unwrap().0 {
        //         heap.pop();
        //         heap.push(Reverse(num));
        //     }
        // }
        // heap.peek().unwrap().0
    }
}
// @lc code=end
