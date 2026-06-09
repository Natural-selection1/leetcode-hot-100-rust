/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] 二叉树展开为链表
*/

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

impl crate::Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        dfs(root, &mut None);
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, head: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut current_node = match node {
        Some(node) => node.borrow_mut(),
        None => return,
    };

    dfs(&current_node.right, head);
    dfs(&current_node.left, head);

    current_node.left = None;
    current_node.right = head.take(); // 头插法
    *head = node.clone(); // 现在链表头节点是 node
}
// @lc code=end
