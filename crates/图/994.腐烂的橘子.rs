/*
 * @lc app=leetcode.cn id=994 lang=rust
 *
 * [994] 腐烂的橘子
 *
 * https://leetcode.cn/problems/rotting-oranges/description/
 *
 * algorithms
 * Medium (55.47%)
 * Likes:    1191
 * Dislikes: 0
 * Total Accepted:    502K
 * Total Submissions: 901.5K
 * Testcase Example:  '[[2,1,1],[1,1,0],[0,1,1]]'
 *
 * 在给定的 m x n 网格 grid 中，每个单元格可以有以下三个值之一：
 *
 *
 * 值 0 代表空单元格；
 * 值 1 代表新鲜橘子；
 * 值 2 代表腐烂的橘子。
 *
 *
 * 每分钟，腐烂的橘子 周围 4 个方向上相邻 的新鲜橘子都会腐烂。
 *
 * 返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，返回 -1 。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：grid = [[2,1,1],[1,1,0],[0,1,1]]
 * 输出：4
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [[2,1,1],[0,1,1],[1,0,1]]
 * 输出：-1
 * 解释：左下角的橘子（第 2 行， 第 0 列）永远不会腐烂，因为腐烂只会发生在 4 个方向上。
 *
 *
 * 示例 3：
 *
 *
 * 输入：grid = [[0,2]]
 * 输出：0
 * 解释：因为 0 分钟时已经没有新鲜橘子了，所以答案就是 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 10
 * grid[i][j] 仅为 0、1 或 2
 *
 *
 */

// @lc code=start
impl crate::Solution {
    // 只能bfs
    fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        const directions: [(i32, i32); 4] = [(0,1),(1,0), (0, -1), (-1, 0)];
        let length = grid[0].len();
        let width = grid.len();

        let mut fresh_count = 0;
        let mut round = 0;
        let mut rotted: Vec<(usize, usize)> = vec![];

        for i in 0..width {
            for j in 0..length {
                match grid[i][j] {
                    1 => { fresh_count += 1 }
                    2 => { rotted.push((i, j)) }
                    _ => {}
                }
            }
        }

        if fresh_count == 0 {
            return 0;
        }

        while !rotted.is_empty() {
            let mut next_rotted: Vec<(usize, usize)> = vec![];
            for _ in 0..rotted.len() {
                let (i, j) = rotted.pop().unwrap();

                for (dx, dy) in directions {
                    let new_i = (i as i32 + dx) as usize;
                    let new_j = (j as i32 + dy) as usize;

                    if (0..width).contains(&new_i)
                    && (0..length).contains(&new_j)
                    && grid[new_i][new_j] == 1 {
                        next_rotted.push((new_i, new_j));
                        grid[new_i][new_j] = 2;
                        fresh_count -= 1;
                    }
                }
            }
            if !next_rotted.is_empty() {
                round += 1;
            }
            rotted = next_rotted;
        }

        if fresh_count == 0 {
            round
        } else {
            -1
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

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut fresh_orange_count: i32 = 0;
    let mut rotten_orange = vec![];

    // 收集初始烂橘子坐标(用于感染)和好橘子总数(用于判定)
    for (i, row) in grid.iter().enumerate() {
        for (j, &orange_state) in row.iter().enumerate() {
            match orange_state {
                1 => fresh_orange_count += 1,
                2 => rotten_orange.push((i, j)),
                _ => {}
            }
        }
    }

    let mut rot_round = 0;
    while fresh_orange_count.is_positive() && !rotten_orange.is_empty() {
        let mut rotting_orange = vec![];

        for (x, y) in rotten_orange {
            let four_direction_coordinate = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (i, j) in four_direction_coordinate {
                if grid.get(i).and_then(|row| row.get(j)) == Some(&1) {
                    fresh_orange_count -= 1;
                    grid[i][j] = 2;
                    rotting_orange.push((i, j));
                }
            }
        }

        rot_round += 1;
        rotten_orange = rotting_orange;
    }

    if fresh_orange_count.is_positive() {
        -1
    } else {
        rot_round
    }
}

*/
