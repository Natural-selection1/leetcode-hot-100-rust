/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 */

// @lc code=start
impl crate::Solution {
    pub fn exist(mut board: Vec<Vec<char>>, target_word: String) -> bool {
        // 预处理, 将所有的数据都装成储存usize
        let mut board: Vec<Vec<usize>> = board
            .iter_mut()
            .map(|row| row.iter_mut().map(|char| *char as usize).collect())
            .collect();
        let mut target_word: Vec<usize> = target_word.chars().map(|char| char as usize).collect();

        let chars_count = board
            .iter()
            .flatten()
            .fold([0; 128], |mut chars_count, &char| {
                chars_count[char] += 1;
                chars_count
            });

        // 1. 可行性剪枝: 检查 target_word 是否有字符数目超了
        if target_word
            .iter()
            .try_fold([0; 128], |mut target_chars_count, &char| {
                target_chars_count[char] += 1;
                match target_chars_count[char] > chars_count[char] {
                    false => Some(target_chars_count),
                    true => None,
                }
            })
            .is_none()
        {
            return false;
        }

        // 2. 顺序剪枝: 优先搜索出现次数更少的字符
        if Iterator::zip(target_word.iter(), target_word.iter().rev())
            .take(target_word.len() / 2)
            .any(|(&l, &r)| chars_count[l] > chars_count[r])
        {
            target_word.reverse();
        }

        let row_count = board.len();
        let col_count = board[0].len();
        for i in 0..row_count {
            for j in 0..col_count {
                if dfs(&mut board, &target_word, i, j, 0, row_count, col_count) {
                    return true;
                }
            }
        }
        false
    }
}

fn dfs(
    board: &mut Vec<Vec<usize>>,
    word: &[usize],
    i: usize,
    j: usize,
    already_found: usize,
    row_count: usize,
    col_count: usize,
) -> bool {
    match (
        board[i][j] == word[already_found],
        already_found + 1 == word.len(),
    ) {
        (false, _) => return false,
        (true, true) => return true,
        _ => {}
    }

    board[i][j] = b'*' as usize;
    for (row, col) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
        if row < row_count
            && col < col_count
            && dfs(
                board,
                word,
                row,
                col,
                already_found + 1,
                row_count,
                col_count,
            )
        {
            return true;
        }
    }
    board[i][j] = word[already_found];

    false
}
// @lc code=end
