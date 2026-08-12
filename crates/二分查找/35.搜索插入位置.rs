/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start


impl crate::Solution {
    // 标准库
    pub fn search_insert_(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }

    // 手写
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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

pub fn search_insert_(nums: Vec<i32>, target: i32) -> i32 {
    nums.partition_point(|&num| num < target) as _
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // 左闭右开区间 [left, right)
    let mut left = 0;
    let mut right = nums.len();

    // 循环不变量：
    // nums[left-1] < target
    // nums[right] >= target
    while left < right {
        let mid = (left + right) / 2;

        match nums[mid].cmp(&target) {
            Ordering::Greater => right = mid,
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return mid as _,
        }
    }

    left as _
}

*/
