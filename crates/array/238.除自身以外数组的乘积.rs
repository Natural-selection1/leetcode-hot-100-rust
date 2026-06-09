/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 */

// @lc code=start
impl crate::Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let postfix_products = {
            let mut postfix_products: Vec<i32> = nums
                .iter()
                .rev()
                .scan(1, |product, num| {
                    *product *= num;
                    Some(*product)
                })
                .collect();
            postfix_products = postfix_products.into_iter().rev().collect();
            postfix_products
        };

        let mut prefix_product = 1;
        let mut answer = Vec::with_capacity(nums_len);
        for (prefix_index, num) in nums.iter().enumerate() {
            let postfix_product = *postfix_products.get(prefix_index + 1).unwrap_or(&1);
            answer.push(prefix_product * postfix_product);
            prefix_product *= num;
        }

        answer
    }
}
// @lc code=end
