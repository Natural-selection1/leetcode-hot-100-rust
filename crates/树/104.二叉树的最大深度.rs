/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_depth: &mut i32, max_depth: &mut i32) {
    todo!()
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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    dfs(&root, &mut 0, &mut max_depth);
    max_depth
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_depth: &mut i32, max_depth: &mut i32) {
    let node = match node {
        Some(node) => node.borrow(),
        None => {
            *max_depth = Ord::max(*max_depth, *current_depth);
            return;
        }
    };

    *current_depth += 1;

    dfs(&node.left, current_depth, max_depth);
    dfs(&node.right, current_depth, max_depth);

    *current_depth -= 1;
}

*/
