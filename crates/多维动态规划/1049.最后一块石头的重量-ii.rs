/*
 * @lc app=leetcode.cn id=1049 lang=rust
 *
 * [1049] 最后一块石头的重量 II
 *
 * https://leetcode.cn/problems/last-stone-weight-ii/description/
 *
 * algorithms
 * Medium (71.98%)
 * Likes:    1075
 * Dislikes: 0
 * Total Accepted:    273.9K
 * Total Submissions: 380.6K
 * Testcase Example:  '[2,7,4,1,8,1]'
 *
 * 有一堆石头，用整数数组 stones 表示。其中 stones[i] 表示第 i 块石头的重量。
 *
 * 每一回合，从中选出任意两块石头，然后将它们一起粉碎。假设石头的重量分别为 x 和 y，且 x <= y。那么粉碎的可能结果如下：
 *
 *
 * 如果 x == y，那么两块石头都会被完全粉碎；
 * 如果 x != y，那么重量为 x 的石头将会完全粉碎，而重量为 y 的石头新重量为 y-x。
 *
 *
 * 最后，最多只会剩下一块 石头。返回此石头 最小的可能重量 。如果没有石头剩下，就返回 0。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：stones = [2,7,4,1,8,1]
 * 输出：1
 * 解释：
 * 组合 2 和 4，得到 2，所以数组转化为 [2,7,1,8,1]，
 * 组合 7 和 8，得到 1，所以数组转化为 [2,1,1,1]，
 * 组合 2 和 1，得到 1，所以数组转化为 [1,1,1]，
 * 组合 1 和 1，得到 0，所以数组转化为 [1]，这就是最优值。
 *
 *
 * 示例 2：
 *
 *
 * 输入：stones = [31,26,33,21,40]
 * 输出：5
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= stones.length <= 30
 * 1 <= stones[i] <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }

        let sum: i32 = stones.iter().sum();
        let sum = sum as usize;
        let half_sum = sum / 2;
        let max = half_sum as usize;

        let mut stones: Vec<usize> = stones
            .into_iter()
            .filter(|x| *x <= max as i32)
            .map(|x| x as usize)
            .collect();
        stones.sort();

        // 即求, 在不超过 max 的容量下, 最多能装多少

        // dp数组含义:
        // 当最大容量为 j 时, 任意stores[0..=i], 最多能装 dp[i][j] 的重量

        // 递推公式:
        // dp[i][j] = max(
        //    dp[i - 1][j], // 不选这个数
        //    dp[i - 1][j - stones[i]] // 选了这个数
        // )

        // 初始化
        let mut dp = vec![vec![0; max + 1]; stones.len()];
        for j in 0..=max {
            if j >= stones[0] {
                dp[0][j] = stones[0];
            }
        }

        for i in 0..stones.len() {
            for j in 0..(max + 1) {
                let mut best = dp[i][j];
                if i >= 1 {
                    best = best.max(dp[i - 1][j]); // 不选
                }
                if i >= 1 && j >= stones[i] {
                    best = best.max(dp[i - 1][j - stones[i]] + stones[i]) // 选了
                }
                dp[i][j] = best
            }
        }

        fprint(&dp);

        let other = sum - dp[stones.len() - 1][max];
        let result = other - dp[stones.len() - 1][max];

        println!("{}", other);
        println!("{}", result);
        result as i32
    }
}

use std::fmt::Debug;
fn fprint<T: Debug>(dp: &Vec<Vec<T>>) {
    for row in dp {
        println!("{:?}", row);
    }

    println!();
}
// @lc code=end
