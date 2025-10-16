/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
 */

// @lc code=start

#![allow(static_mut_refs)]
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl crate::Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let val_inorder_index_map = inorder
            .iter()
            .enumerate()
            .map(|(index, &val)| (val, index))
            .collect();

        dfs(0, preorder.len(), 0, &preorder, &val_inorder_index_map)
    }
}

fn dfs(
    preorder_index: usize,
    preorder_right_boundary: usize,
    inorder_left_boundary: usize, // * 用来算左子树长度的
    preorder: &[i32],
    val_inorder_index_map: &HashMap<i32, usize>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // 使用左闭右开区间
    if preorder_index == preorder_right_boundary {
        return None;
    }
    let val = preorder[preorder_index];
    let inorder_index = val_inorder_index_map[&val];

    let l_tree_size = inorder_index - inorder_left_boundary;
    let r_preorder_index = preorder_index + l_tree_size + 1;

    let l_node = dfs(
        preorder_index + 1,
        r_preorder_index,
        inorder_left_boundary,
        preorder,
        val_inorder_index_map,
    );
    let r_node = dfs(
        r_preorder_index,
        preorder_right_boundary,
        inorder_index + 1,
        preorder,
        val_inorder_index_map,
    );

    Some(Rc::new(RefCell::new(TreeNode {
        val: preorder[preorder_index],
        left: l_node,
        right: r_node,
    })))
}

// @lc code=end
