/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */

// @lc code=start
impl crate::Solution {
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
// @lc code=end
