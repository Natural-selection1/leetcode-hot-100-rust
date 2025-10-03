/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 */

// @lc code=start
impl crate::Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = i32::MIN;
        // 由于负负得正, 所以需要记录最小值
        let mut recent_max_product = 1;
        let mut recent_min_product = 1;

        #[allow(clippy::unwrap_used, reason = "所给切片不为空")]
        for num in nums {
            let alternative_nums = [num, num * recent_max_product, num * recent_min_product];
            let max = *alternative_nums.iter().max().unwrap();
            let min = *alternative_nums.iter().min().unwrap();

            max_product = max_product.max(max);
            (recent_max_product, recent_min_product) = (max, min);
        }

        max_product
    }
}
// @lc code=end
