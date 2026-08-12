/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start
impl crate::Solution {
    pub fn rob(values: Vec<i32>) -> i32 {
        todo!()
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

pub fn rob(values: Vec<i32>) -> i32 {
        values
            .iter()
            .skip(1)
            .fold(
                (0, values[0]),
                // 状态转移:
                //     dp[i]   = max(dp[i-1], dp[i-2] + val)
                |(n1, n2), val| {
                    // dp[i+1] = max(dp[i]  , dp[i-1] + val)
                    (n2, n2.max(n1 + val))
                },
            )
            .1
    }
}
*/
