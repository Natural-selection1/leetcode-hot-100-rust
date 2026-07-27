/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start
impl crate::Solution {
    pub fn coin_change(coins: Vec<i32>, target_amount: i32) -> i32 {
        let coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
        let target_amount = target_amount as usize;

        let mut min_coins_of = vec![usize::MAX / 2; target_amount + 1];
        min_coins_of[0] = 0;

        // 以 coins = [5, 1, 2], amount = 11 举例
        // 列标:               0  1   2   3   4   5   6   7   8   9   10  11
        // 初始化:             0  ∞  ∞   ∞  ∞  ∞   ∞  ∞   ∞  ∞  ∞  ∞
        // 5:                  0  ∞  ∞   ∞  ∞  1   ∞   ∞  ∞   ∞  ∞  ∞
        // 1:                  0  1   2   3   4   1   2   3   4   5   2   3
        // 2:                  0  1   1   2   2   1   2   2   3   3   2   3
        for coin in coins {
            for amount in coin..=target_amount {
                min_coins_of[amount] = min_coins_of[amount].min(min_coins_of[amount - coin] + 1);
            }
        }

        match min_coins_of[target_amount] > target_amount {
            true => -1,
            false => min_coins_of[target_amount] as _,
        }
    }
}
// @lc code=end
