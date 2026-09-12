/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl crate::Solution {
    // 二维dp
    pub fn unique_paths(length: i32, width: i32) -> i32 {
        let length = length as usize;
        let width = width as usize;
        // dp数组含义:
        // dp[i][j] 是 在第i行, 第j列的路径数

        // 递推公式:
        // dp[i][j] = dp[i-1][j] + dp[i][j-1]

        // 初始化: 全1, 反正本地位置是用不上的
        let row = vec![1; length];
        let mut dp = vec![row; width];

        // 遍历顺序: 外层上下, 内层左右

        for i in 1..width {
            for j in 1..length {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
            }
        }

        dp[width - 1][length - 1]
    }

    // 一维dp
    pub fn unique_paths_(length: i32, width: i32) -> i32 {
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

pub fn unique_paths(length: i32, width: i32) -> i32 {
    if length.min(width) == 1 {
        return 1;
    }

    let length = length as usize;
    let width = width as usize;
    let mut dp = vec![vec![1; width]; length];

    for i in 1..length {
        for j in 1..width {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[length - 1][width - 1]
}

pub fn unique_paths_(length: i32, width: i32) -> i32 {
    if length.min(width) == 1 {
        return 1;
    }

    let length = length as usize;
    let width = width as usize;
    let mut dp = vec![1; length];

    (1..width).for_each(|_| (1..length).for_each(|i| dp[i] += dp[i - 1]));

    dp[length - 1]
}

*/
