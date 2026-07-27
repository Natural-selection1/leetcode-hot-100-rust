/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer = vec![];
        dfs(&root, &mut answer);
        answer
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
    let node = match node {
        None => return,
        Some(node) => node.borrow(),
    };

    dfs(&node.left, answer);
    answer.push(node.val);
    dfs(&node.right, answer);
}
// @lc code=end
