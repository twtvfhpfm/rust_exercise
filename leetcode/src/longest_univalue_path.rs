//Definition for a binary tree node.
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;
        Solution::node_length(root.as_ref(), &mut result);
        result
    }

    fn node_length(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Solution::node_length(node.borrow().left.as_ref(), result);
                let right = Solution::node_length(node.borrow().right.as_ref(), result);
                let mut left_length = 0;
                let mut right_length = 0;
                if let Some(left_node) = node.borrow().left.as_ref(){
                    if left_node.borrow().val == node.borrow().val {
                        left_length = left + 1;
                    }
                }
                if let Some(right_node) = node.borrow().right.as_ref(){
                    if right_node.borrow().val == node.borrow().val {
                        right_length = right + 1;
                    }
                }

                *result = (*result).max(left_length + right_length);
                left_length.max(right_length)

            }
        }
    }
}