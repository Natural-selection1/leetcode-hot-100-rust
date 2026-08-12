/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl crate::Solution {
    pub fn climb_stairs(times: i32) -> i32 {
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

pub fn climb_stairs(times: i32) -> i32 {
        (1..=times)
            .fold(
                (0, 1),
                // 状态转移:
                // dp[i]       = dp[i-2] + dp[i-1]
                |(n1, n2), _| {
                    // dp[i+1] = dp[i-1] + (dp[i-2] + dp[i-1])
                    (n2, n1 + n2)
                },
            )
            .1
    }
}
*/
