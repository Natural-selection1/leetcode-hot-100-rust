/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 *
 * https://leetcode.cn/problems/word-search/description/
 *
 * algorithms
 * Medium (50.68%)
 * Likes:    2221
 * Dislikes: 0
 * Total Accepted:    945.6K
 * Total Submissions: 1.8M
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。
 * 如果 word 存在于网格中，返回 true ；否则，返回 false
 * 。
 *
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，
 * 其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。
 * 同一个单元格内的字母不允许被重复使用。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word =
 * "ABCCED"
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word =
 * "SEE"
 * 输出：true
 *
 *
 * 示例 3：
 *
 *
 * 输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word =
 * "ABCB"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n = board[i].length
 * 1 <= m, n <= 6
 * 1 <= word.length <= 15
 * board 和 word 仅由大小写英文字母组成
 *
 *
 *
 *
 * 进阶：你可以使用搜索剪枝的技术来优化解决方案，
 * 使其在 board 更大的情况下可以更快解决问题？
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn exist(board: Vec<Vec<char>>, target_word: String) -> bool {
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
            if target_chars_count[char] <= chars_count[char] {
                Some(target_chars_count)
            } else {
                None
            }
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
            if find_it(&mut board, &target_word, i, j, 0, row_count, col_count) {
                return true;
            }
        }
    }

    false
}

fn find_it(
    board: &mut Vec<Vec<usize>>,
    target_word: &[usize],
    i: usize,
    j: usize,
    next_find: usize,
    row_count: usize,
    col_count: usize,
) -> bool {
    if board[i][j] != target_word[next_find] {
        return false;
    }
    if next_find + 1 == target_word.len() {
        return true;
    }
    board[i][j] = b'*' as usize;

    for (row, col) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
        if row < row_count
            && col < col_count
            && find_it(
                board,
                target_word,
                row,
                col,
                next_find + 1,
                row_count,
                col_count,
            )
        {
            return true;
        }
    }
    board[i][j] = target_word[next_find];

    false
}

*/
