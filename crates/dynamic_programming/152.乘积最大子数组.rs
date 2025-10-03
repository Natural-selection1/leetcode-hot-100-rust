/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 */

// @lc code=start
impl crate::Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = i32::MIN;
        let mut last_one_max_product = 1;
        let mut last_one_min_product = 1;

        for current_num in nums {
            (last_one_max_product, last_one_min_product) = (
                current_num
                    .max(last_one_max_product * current_num)
                    .max(last_one_min_product * current_num),
                current_num
                    .min(last_one_max_product * current_num)
                    .min(last_one_min_product * current_num),
            );
            max_product = max_product.max(last_one_max_product);
        }

        max_product
    }
}
// @lc code=end
