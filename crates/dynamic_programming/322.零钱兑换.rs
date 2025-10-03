/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start
impl crate::Solution {
    pub fn coin_change(mut coins: Vec<i32>, target_amount: i32) -> i32 {
        let coins: Vec<usize> = coins.iter_mut().map(|x| *x as usize).collect();
        let target_amount = target_amount as usize;

        let mut min_coins_of = vec![usize::MAX / 2; target_amount + 1];
        min_coins_of[0] = 0;

        for coin in coins {
            for i in coin..=target_amount {
                min_coins_of[i] = min_coins_of[i].min(min_coins_of[i - coin] + 1);
            }
        }

        match min_coins_of[target_amount] > target_amount {
            true => -1,
            false => min_coins_of[target_amount] as _,
        }
    }
}
// @lc code=end
