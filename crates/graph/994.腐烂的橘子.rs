/*
 * @lc app=leetcode.cn id=994 lang=rust
 *
 * [994] 腐烂的橘子
 */

// @lc code=start
impl crate::Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh_orange_count: i32 = 0;
        let mut rotten_orange = vec![];

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
}
// @lc code=end
