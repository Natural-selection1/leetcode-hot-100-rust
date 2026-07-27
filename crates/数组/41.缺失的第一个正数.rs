/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
const UNREACHABLE: usize = 0;

impl crate::Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.push(0); // 在末尾添加一个无关紧要的数，让索引从1开始对应
        let mut nums: Vec<usize> = nums
            .iter()
            .map(|&num| {
                let num = num as usize;
                // 只保留可能的正数, 超过范围的全部转成UNREACHABLE
                match (1..=len).contains(&num) {
                    true => num,
                    false => UNREACHABLE,
                }
            })
            .collect();

        // 把遍历到的数, 搬到它应该在的位置(即其数值所表达的下标)
        for index in 0..len {
            let mut current_num = nums[index];
            while !(
                // 不可达的不动
                current_num == UNREACHABLE
                // 已经符合当前位置的不动
                || current_num == nums[current_num]
            ) {
                // 我们总是会把一个数放到属于它的位置上
                std::mem::swap(&mut nums[current_num], &mut current_num);
            }
        }
        // 现在 nums[1] 对应数值1，nums[2] 对应数值2

        (1..=len).find(|&i| nums[i] != i).unwrap_or(len + 1) as i32
    }
}
// @lc code=end
