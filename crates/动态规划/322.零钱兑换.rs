/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 *
 * https://leetcode.cn/problems/coin-change/description/
 *
 * algorithms
 * Medium (52.61%)
 * Likes:    3299
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 2.7M
 * Testcase Example:  '[1,2,5]\n11'
 *
 * 给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。
 *
 * 计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。
 *
 * 你可以认为每种硬币的数量是无限的。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：coins = [1, 2, 5], amount = 11
 * 输出：3
 * 解释：11 = 5 + 5 + 1
 *
 * 示例 2：
 *
 *
 * 输入：coins = [2], amount = 3
 * 输出：-1
 *
 * 示例 3：
 *
 *
 * 输入：coins = [1], amount = 0
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= coins.length <= 12
 * 1 <= coins[i] <= 2^31 - 1
 * 0 <= amount <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, target_amount: i32) -> i32 {
        let target_amount = target_amount as usize;
        let coins: Vec<usize> = coins.into_iter().map(|n| n as usize).collect();

        // 标准完全背包问题

        /*
        dp数组含义:
        当 target 为 j 时, 最少能凑成的总数

        递推公式:
        dp[j] = dp[j].min(dp[j - coins[k]] + 1)

        初始化:
        0下标: dp[0] = 0
        非0下标: dp[j] =
         */

        let mut dp = vec![usize::MAX / 2; target_amount + 1];
        dp[0] = 0;

        for coin in coins {
            for j in coin..(target_amount + 1) {
                dp[j] = dp[j].min(dp[j - coin] + 1)
            }
        }

        if dp[target_amount] == usize::MAX / 2 {
            -1
        } else {
            dp[target_amount] as i32
        }
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

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
*/
