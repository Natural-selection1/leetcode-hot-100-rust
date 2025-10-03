/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl crate::Solution {
    pub fn majority_element_(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major_num = 0;
        let mut count = 0;

        for num in nums {
            match (count, num == major_num) {
                (0, _) => (major_num, count) = (num, 1),
                (_, true) => count += 1,
                (_, false) => count -= 1,
            }
        }
        major_num
    }
}
// @lc code=end
