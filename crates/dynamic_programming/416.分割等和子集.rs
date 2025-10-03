/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */

// @lc code=start
impl crate::Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.into_iter().map(|num| num as usize).collect();
        let nums_sum: usize = nums.iter().sum();
        let target_sum = nums_sum / 2;

        // 可行性检查
        if !nums_sum.is_multiple_of(2) || nums.iter().max() > Some(&target_sum) {
            return false;
        }

        let mut can_build_sum = vec![false; target_sum + 1];
        can_build_sum[0] = true;

        // 其实就是0-1背包
        for num in nums {
            for sum in (0..=target_sum).rev() {
                if sum >= num {
                    can_build_sum[sum] |= can_build_sum[sum - num];
                }
            }

            if can_build_sum[target_sum] {
                return true;
            }
        }

        can_build_sum[target_sum]
    }
}
// @lc code=end
