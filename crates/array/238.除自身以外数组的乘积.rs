/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 */

// @lc code=start
impl crate::Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();

        let mut exist_product = 1;
        let mut postfix_products = vec![0; nums_len];
        for (postfix_index, num) in nums.iter().rev().enumerate() {
            let new_product = exist_product * num;
            postfix_products[nums_len - 1 - postfix_index] = new_product;
            exist_product = new_product;
        }

        let mut exist_product = 1;
        let mut answer = Vec::with_capacity(nums_len);
        for (prefix_index, num) in nums.iter().enumerate() {
            let postfix_product = *postfix_products.get(prefix_index + 1).unwrap_or(&1);
            answer.push(exist_product * postfix_product);
            exist_product *= num;
        }

        answer
    }
}
// @lc code=end
