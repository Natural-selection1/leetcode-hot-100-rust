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
        dfs_1(root);
        // dfs_2(root, &mut None);
    }
}

fn dfs_1(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut root = match root {
        Some(node) => node.borrow_mut(),
        None => return,
    };

    // 把左右子树取下来各自捋直
    let mut left = root.left.take();
    let mut right = root.right.take();
    dfs_1(&mut left);
    dfs_1(&mut right);

    match (left, right) {
        (None, None) => return,
        (branch @ Some(_), None) | (None, branch @ Some(_)) => return root.right = branch,
        (left @ Some(_),  right @ Some(_)) => {
            // 先把左子树插上去, 再把右子树刚刚插上去的左子树的尾部
            root.right = left;

            // 找到链表的末尾
            let mut current = root.right.clone();
            while let Some(node) = current {
                let next = node.borrow().right.clone();
                if next.is_none() {
                    // 找到了最后一个节点，直接修改
                    node.borrow_mut().right = right;
                    return;
                }
                current = next;
            }
        }
    }
}

fn dfs_2(root: &Option<Rc<RefCell<TreeNode>>>, head: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut current_node = match root {
        Some(node) => node.borrow_mut(),
        None => return,
    };

    dfs_2(&current_node.right, head);
    dfs_2(&current_node.left, head);

    current_node.left = None;
    current_node.right = head.take(); // 头插法
    *head = root.clone(); // 现在链表头节点是 node
}
// @lc code=end
