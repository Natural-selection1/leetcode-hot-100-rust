/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 *
 * https://leetcode.cn/problems/number-of-islands/description/
 *
 * algorithms
 * Medium (63.83%)
 * Likes:    3020
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 2.3M
 * Testcase Example:  '[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]'
 *
 * 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
 *
 * 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
 *
 * 此外，你可以假设该网格的四条边均被水包围。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [
 * ['1','1','1','1','0'],
 * ['1','1','0','1','0'],
 * ['1','1','0','0','0'],
 * ['0','0','0','0','0']
 * ]
 * 输出：1
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [
 * ['1','1','0','0','0'],
 * ['1','1','0','0','0'],
 * ['0','0','1','0','0'],
 * ['0','0','0','1','1']
 * ]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 300
 * grid[i][j] 的值为 '0' 或 '1'
 *
 *
 */

// @lc code=start
impl crate::Solution {
    fn num_islands_(islands: Vec<Vec<char>>) -> i32 {
        let length = islands[0].len();
        let width = islands.len();

        let mut result = 0;
        let mut visited = vec![vec![false; length]; width];

        for i in 0..width {
            for j in 0..length {
                if islands[i][j] == '1' && visited[i][j] == false {
                    result += 1;
                    mark_visited_dfs(&islands, &mut visited, i, j);
                }
            }
        }

        result
    }

    // bfs 写法
    fn num_islands(islands: Vec<Vec<char>>) -> i32 {
        let length = islands[0].len();
        let width = islands.len();

        let mut result = 0;
        let mut visited = vec![vec![false; length]; width];

        for i in 0..width {
            for j in 0..length {
                if islands[i][j] == '1' && visited[i][j] == false {
                    result += 1;
                    mark_visited_bfs(&islands, &mut visited, i, j);
                }
            }
        }

        result
    }
}

fn mark_visited_bfs(islands: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    use std::collections::VecDeque;

    const directions : [(i32, i32); 4] = [(0,1), (1,0), (0, -1), (-1, 0)];
    let length = islands[0].len();
    let width = islands.len();

    let mut deque = VecDeque::new();
    deque.push_back((i, j));

    while !deque.is_empty() {
        let (i, j) = deque.pop_back().unwrap();
        visited[i][j] = true;

        for (dx, dy) in directions {
            let new_i = (i as i32 + dx) as usize;
            let new_j = (j as i32 + dy) as usize;

            if (0..width).contains(&new_i)
            && (0..length).contains(&new_j)
            && visited[new_i][new_j] == false
            && islands[new_i][new_j] == '1'
            {
                deque.push_back((new_i, new_j));
            }
        }
    }
}


fn mark_visited_dfs(
    islands: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
) {
    if islands[i][j] == '0' || visited[i][j] == true {
        return;
    }

    visited[i][j] = true;

    const directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0,-1), (-1,0)];
    let length = islands[0].len();
    let width = islands.len();

    for (dx, dy) in directions {
        let new_i = (i as i32 + dx) as usize;
        let new_j = (j as i32 + dy) as usize;

        if (0..width).contains(&new_i) && (0..length).contains(&new_j) {
            mark_visited_dfs(islands, visited, new_i, new_j);
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

pub fn num_islands(mut islands_map: Vec<Vec<char>>) -> i32 {
    let mut islands_count = 0;

    for i in 0..islands_map.len() {
        for j in 0..islands_map[i].len() {
            if islands_map[i][j] == '1' {
                visit_island(&mut islands_map, i, j);
                islands_count += 1;
            }
        }
    }

    islands_count
}

fn visit_island(islands_map: &mut Vec<Vec<char>>, row: usize, col: usize) {
    if islands_map.get(row).and_then(|row| row.get(col)) != Some(&'1') {
        return;
    }

    islands_map[row][col] = '*'; // 标记已访问
                                 // 1 0 1 1 1
                                 // 1 0 1 0 1
                                 // 1 0 0 0 1
                                 // 1 1 1 1 1
                                 // 不能只向右和下走, 是为了防止出现上面这种情况
    visit_island(islands_map, row, col - 1);
    visit_island(islands_map, row, col + 1);
    visit_island(islands_map, row - 1, col);
    visit_island(islands_map, row + 1, col);
}

*/
