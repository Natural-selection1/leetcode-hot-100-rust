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

        if !nums_sum.is_multiple_of(2) || *nums.iter().max().unwrap() > target_sum {
            return false;
        }

        let mut is_sum_can_be_built = vec![false; target_sum + 1];
        is_sum_can_be_built[0] = true;

        for num in nums {
            for sum in (0..=target_sum).rev() {
                if sum >= num {
                    is_sum_can_be_built[sum] |= is_sum_can_be_built[sum - num];
                }
            }
            if is_sum_can_be_built[target_sum] {
                return true;
            }
        }

        is_sum_can_be_built[target_sum]
    }
}
// @lc code=end
