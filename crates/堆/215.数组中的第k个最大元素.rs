/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start


impl crate::Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // 耗时O(n)的原地转换
    let mut heap = BinaryHeap::from(nums);
    for _ in 0..(k - 1) {
        heap.pop();
    }
    *heap.peek().unwrap()
}

*/
