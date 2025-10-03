/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl crate::Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // * 前/后缀
        // let mut prefix_min = vec![];
        // let mut min = i32::MAX;
        // for preice in &prices {
        //     min = min.min(*preice);
        //     prefix_min.push(min);
        // }
        // let mut postfix_max = Vec::with_capacity(prices.len());
        // let mut max = i32::MIN;
        // for &price in prices.iter().rev() {
        //     max = max.max(price);
        //     postfix_max.push(max);
        // }
        // postfix_max.reverse();
        // let mut max = i32::MIN;
        // for (post, pre) in postfix_max.iter().zip(prefix_min.iter()) {
        //     max = max.max(*post - *pre)
        // }
        // max

        // * 贪心
        prices
            .iter()
            .fold((0, i32::MAX), |(max_profit, min_price), &price| {
                (max_profit.max(price - min_price), min_price.min(price))
            })
            .0
    }
}
// @lc code=end
