/*
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * [136] 只出现一次的数字
 */

// @lc code=start
impl crate::Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
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

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, num| acc ^ num)
}

*/
