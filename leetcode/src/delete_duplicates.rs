// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){
            return None;
        }
        let delete_val = head.unwrap().val - 1;
        let mut new_head = Some(Box::new(ListNode{val: 0, next: None}));
        let mut tail = new_head.as_mut();

        let Some(mut cur) = head.take();
        while cur.next.is_some() {
            if cur.val == delete_val{
                cur = cur.next.take().unwrap();
            }
            else{
                let next_val = cur.next.as_ref().unwrap().val;
                if cur.val == next_val{
                    delete_val = cur.val;
                }
                else{
                    tail.unwrap().next = Some(cur);
                    cur = cur.next.take().unwrap();
                }

            }

        }
        if cur.val != delete_val{
            tail.unwrap().next = Some(cur);
        }

        return new_head.unwrap().next;

    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode{val:0, next:head}));
        let mut ptr = dummy_head.as_mut().unwrap();
        while ptr.next.is_some() {
            // 拿到下一个node 的所有权，然后放到下面的函数处理，如果返回(None, Some)说明
            // 前一个重复了，直接把后一个接到当前节点上，但当前节点指针不向后滑
            // 如果前一个没重复，就把前一个接到当前节点上，并且指针向后滑，再把后一个接到当前节点上
            let next = ptr.next.take();
            let (p1, p2) = Self::delete_rec(next);
            if p1.is_some() {
                ptr.next = p1;
                ptr = ptr.next.as_mut().unwrap();
                ptr.next = p2;
            } else {
                ptr.next = p2;
            }
            
        }
        dummy_head.as_mut().unwrap().next.take()
    }
    pub fn delete_rec(mut ptr:Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        // 此函数主要返回当前node和node.next, 原则是如果ptr节点后面有重复内容，则返回(None, next)
        // 如果没有重复，则返回(ptr, next)
        // 最关键的一点是把当前节点之后的内链表打断，变成两部分，前一部分重复就抛弃，不重复就返回
        if ptr.is_none() {
            return (None, None);
        }
        let mut next = ptr.as_mut().unwrap().next.take();
        let val = ptr.as_ref().unwrap().val;
        let mut flag = false;
        while next.is_some() {
            if next.as_mut().unwrap().val == val {
                flag = true;
                next = next.as_mut().unwrap().next.take();
            } else {
                break;
            }
        }
        if flag {
            return (None, next);
        } else {
            return (ptr, next);
        }
    }
}