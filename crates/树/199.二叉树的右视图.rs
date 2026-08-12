/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * [199] 二叉树的右视图
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: &mut usize, vec_right_view: &mut Vec<i32>) {
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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vec_right_view = Vec::with_capacity(100);
    dfs(&root, &mut 0, &mut vec_right_view);
    vec_right_view
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: &mut usize, vec_right_view: &mut Vec<i32>) {
    let node = match node {
        Some(node) => node.borrow(),
        None => return,
    };

    if vec_right_view.len() == *depth {
        vec_right_view.push(node.val);
    }
    *depth += 1;

    dfs(&node.right, depth, vec_right_view);
    dfs(&node.left, depth, vec_right_view);

    *depth -= 1;
}

*/
