/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl crate::Solution {
    // 贪心
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold(
                (0, i32::MAX / 2),
                |(max_profit, min_price), &current_price| {
                    (
                        max_profit.max(current_price - min_price),
                        min_price.min(current_price),
                    )
                },
            )
            .0
    }

    // 前/后缀
    pub fn max_profit_(prices: Vec<i32>) -> i32 {
        let prefix_min: Vec<i32> = prices
            .iter()
            .scan(i32::MAX, |min, &price| {
                *min = (*min).min(price);
                Some(*min)
            })
            .collect();

        let postfix_max: Vec<i32> = prices
            .iter()
            .rev()
            .scan(i32::MIN, |max, &price| {
                *max = (*max).max(price);
                Some(*max)
            })
            .collect();

        #[allow(clippy::unwrap_used, reason = "前后缀数组不为空")]
        Iterator::zip(postfix_max.iter().rev(), prefix_min.iter())
            .map(|(post, pre)| post - pre)
            .max()
            .unwrap()
    }
}
// @lc code=end
