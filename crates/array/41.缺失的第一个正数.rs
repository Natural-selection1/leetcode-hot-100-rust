/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
const UNREACHABLE: usize = 0;

impl crate::Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        // 在末尾添加一个无关紧要的数，让索引从1开始对应
        nums.push(i32::MAX);
        let mut nums: Vec<usize> = nums
            .iter()
            .map(|&num| {
                // 只保留可能的正数, 超过范围的全部转成UNREACHABLE
                let num = num as usize;
                match (1..=nums_len).contains(&num) {
                    true => num,
                    false => UNREACHABLE,
                }
            })
            .collect();

        // 把遍历到的数, 搬到它应该在的位置(即其数值所表达的下标)
        for index in 0..nums_len {
            let mut num = nums[index];
            while !(num == UNREACHABLE || num == nums[num]) {
                std::mem::swap(&mut nums[num], &mut num);
            }
        }
        // 现在 nums[1] 对应数值1，nums[2] 对应数值2

        (1..=nums_len)
            .find(|&i| nums[i] != i)
            .unwrap_or(nums_len + 1) as i32
    }
}
// @lc code=end
