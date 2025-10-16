/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] 二叉树展开为链表
*/

use crate::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;

impl crate::Solution {
    pub fn flatten(root: &mut Option<Node>) {
        dfs(root, &mut None);
    }
}

fn dfs(node: &Option<Node>, head: &mut Option<Node>) {
    let Some(current_node) = node else { return };
    let mut current_node = current_node.borrow_mut();
    dfs(&current_node.right, head);
    dfs(&current_node.left, head);
    current_node.left = None;
    current_node.right = head.take(); // 头插法
    *head = node.clone(); // 现在链表头节点是 node
}

// @lc code=end
