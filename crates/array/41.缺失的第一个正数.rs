/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
impl crate::Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        // 把遍历到的数, 搬到它应该在的位置(即其数值所表达的下标)
        for walk_index in 0..nums_len {
            let mut num = nums[walk_index];
            while (1..=nums_len).contains(&(num as usize)) // 跳过非整数
            && nums[(num - 1) as usize] != num
            {
                std::mem::swap(&mut nums[(num - 1) as usize], &mut num);
            }
        }

        nums.iter()
            .enumerate()
            .position(|(walk_index, &num)| num != walk_index as i32 + 1)
            .map(|target_index| target_index as i32 + 1)
            // 如果所有数都符合条件, 说明缺失的是 nums_len + 1
            .unwrap_or(nums_len as i32 + 1)
    }
}
// @lc code=end
