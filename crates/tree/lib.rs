#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

#[path = "105.从前中序遍历构造二叉树.rs"]
mod build_tree;
#[path = "543.二叉树的直径.rs"]
mod diameter_of_binary_tree;
#[path = "./114.二叉树展开为链表.rs"]
mod flatten;
#[path = "94.二叉树的中序遍历.rs"]
mod inorder_traversal;
#[path = "226.翻转二叉树.rs"]
mod invert_tree;
#[path = "101.对称二叉树.rs"]
mod is_symmetric;
#[path = "98.验证二叉搜索树.rs"]
mod is_valid_bst;
#[path = "./230.二叉搜索树中第-k-小的元素.rs"]
mod kth_smallest;
#[path = "102.二叉树的层序遍历.rs"]
mod level_order;
#[path = "./236.二叉树的最近公共祖先.rs"]
mod lowest_common_ancestor;
#[path = "104.二叉树的最大深度.rs"]
mod max_depth;
#[path = "./124.二叉树中的最大路径和.rs"]
mod max_path_sum;
#[path = "./437.路径总和-iii.rs"]
mod path_sum;
#[path = "./199.二叉树的右视图.rs"]
mod right_side_view;
#[path = "108.有序数组转二叉搜索树.rs"]
mod sorted_array_to_bst;

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
