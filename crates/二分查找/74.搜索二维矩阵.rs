/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start

impl crate::Solution {
    // 暴力二分
    pub fn search_matrix_(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let target_line = matrix
            .into_iter()
            .find(|row| row.last().expect("行为空") >= &target);

        match target_line {
            Some(x) => (|row: &Vec<i32>| row.binary_search(&target).is_ok())(&x),
            None => false,
        }
    }

    // 利用单调性
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0;
        let mut col_opt = Some(matrix[0].len() - 1);

        loop {
            let col = match col_opt {
                Some(col) => col,
                None => break,
            };
            let val = match matrix.get(row).and_then(|r| r.get(col)) {
                Some(val) => val,
                None => break,
            };

            match val.cmp(&target) {
                std::cmp::Ordering::Greater => col_opt = col.checked_sub(1),
                std::cmp::Ordering::Less => row += 1,
                std::cmp::Ordering::Equal => return true,
            }
        }

        false
    }
}

// @lc code=end
