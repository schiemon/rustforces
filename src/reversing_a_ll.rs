use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display, Formatter};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Debug for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.next {
            write!(f, "{}, {:?}", &self.val, n)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn new_with(val: i32, next: ListNode) -> Self {
        ListNode {
            val,
            next: Some(Box::new(next)),
        }
    }
}

fn rev(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut boxed_node) = curr {
        let mut tmp = boxed_node.next.take();
        curr = tmp;
        boxed_node.next = reversed.take();
        reversed = Some(boxed_node);
    }

    reversed
}

fn rev_range(head: Option<Box<ListNode>>, from: u32, to: u32) -> Option<Box<ListNode>> {
    assert!(from <= to);

    if from == to {
        return head;
    }
    // Skip to the head of the segment that has to be reversed.

    let mut cursor = head;
    let mut pos = 1;

    let mut front: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut front_back = front.as_mut().unwrap();

    while let Some(mut x) = cursor {
        if pos == from {
            cursor = Some(x); // take it back and do not drop it
            break;
        } else {
            pos += 1;
            cursor = x.next.take();
            front_back.next = Some(x);
            front_back = front_back.next.as_mut().unwrap(); // front_back should now be looking at the newly added element
        }
    }

    // `cursor` is now the head of the middle part.
    let mut middle_head = Some(Box::new(ListNode::new(0)));
    let mut middle_tail = middle_head.as_mut().unwrap();

    while let Some(mut x) = cursor {
        if pos == to + 1 {
            cursor = Some(x); // take it back and do not drop it
            break;
        } else {
            pos += 1;
            cursor = x.next.take();
            middle_tail.next = Some(x);
            middle_tail = middle_tail.next.as_mut().unwrap(); // middle_back should now be looking at the newly added element
        }
    }

    let head_of_back = cursor;
    let mut ptr = middle_head.as_mut().unwrap().next.as_mut().unwrap() as *mut Box<ListNode>;
    middle_head = rev(middle_head.unwrap().next);
    // SAFETY:
    unsafe {
        (*ptr).next = head_of_back;
    }
    front_back.next = middle_head;

    return front.unwrap().next;
}
struct HeadTail<'a> {
    head: Box<ListNode>,
    tail: &'a mut Box<ListNode>,
}

fn main() {
    println!("Starting playground...");
    let ll = get_sample_list();
    println!("Original: {:?}", ll);
    println!("Reversed: {:?}", rev(ll));
}

fn get_sample_list() -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new_with(
        1,
        ListNode::new_with(
            2,
            ListNode::new_with(
                3,
                ListNode::new_with(
                    4,
                    ListNode::new_with(
                        5,
                        ListNode::new_with(6, ListNode::new_with(7, ListNode::new(8))),
                    ),
                ),
            ),
        ),
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testSmth() {
        let mut ll = get_sample_list();
        let from = 3u32;
        let to = 4u32;
        let mut s1 = serialize(&ll);
        &s1[from as usize - 1..to as usize].reverse();

        let mut s2 = serialize(&rev_range(ll, from, to));

        assert_eq!(s1, s2);

        println!("{:?} {:?}", s1, s2);
    }

    fn serialize(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut s = Vec::<i32>::new();

        let mut curr = head;

        while let Some(x) = curr {
            s.push(x.val);
            curr = &x.next;
        }

        s
    }
}
