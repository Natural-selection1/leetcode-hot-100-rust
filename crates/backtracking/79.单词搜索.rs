/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 */

// @lc code=start
impl crate::Solution {
    pub fn exist(board: Vec<Vec<char>>, target_word: String) -> bool {
        // 预处理, 转成usize方便后续计算
        let mut board: Vec<Vec<usize>> = board
            .into_iter()
            .map(|row| row.into_iter().map(|char| char as usize).collect())
            .collect();
        let mut target_word: Vec<usize> = target_word.chars().map(|char| char as usize).collect();

        // 统计每个字符出现的次数, 以进行剪枝
        let chars_count = board
            .iter()
            .flatten()
            .fold([0; 128], |mut chars_count, &char| {
                chars_count[char] += 1;
                chars_count
            });

        // 可行性剪枝: 检查 target_word 是否有字符数目超了
        if target_word
            .iter()
            .try_fold([0; 128], |mut target_chars_count, &char| {
                target_chars_count[char] += 1;
                (target_chars_count[char] <= chars_count[char]).then_some(target_chars_count)
            })
            .is_none()
        {
            return false;
        }

        // 顺序剪枝: 优先搜索出现次数更少的字符
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
    target_word: &[usize],
    i: usize,
    j: usize,
    already_found: usize,
    row_count: usize,
    col_count: usize,
) -> bool {
    if board[i][j] != target_word[already_found] {
        return false;
    }
    if already_found + 1 == target_word.len() {
        return true;
    }
    board[i][j] = b'*' as usize;

    for (row, col) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
        if row < row_count
            && col < col_count
            && dfs(
                board,
                target_word,
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
    board[i][j] = target_word[already_found];

    false
}
// @lc code=end
