/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */

// @lc code=start
impl crate::Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        let mut cursor_i = nums.len().checked_sub(2);

        // Step 1: 从后向前查找第一个相邻升序对(i, i+1)
        while let Some(i) = cursor_i
            && nums[i] >= nums[i + 1]
        {
            cursor_i = i.checked_sub(1);
        }

        // 如果找到了升序对，执行Step 2和Step 3
        if let Some(i) = cursor_i {
            let mut cursor_j = nums.len().checked_sub(1);
            // Step 2: 从后向前查找第一个大于nums[i]的元素
            while let Some(j) = cursor_j
                && nums[i] >= nums[j]
            {
                cursor_j = j.checked_sub(1);
            }
            // Step 3: 交换nums[i]和nums[j]
            nums.swap(i, cursor_j.unwrap_or(0));
        }

        // Step 4: 翻转i之后的所有元素
        match cursor_i {
            Some(i) => nums[(i + 1)..].reverse(),
            None => nums.reverse(),
        }
    }
}
// @lc code=end
