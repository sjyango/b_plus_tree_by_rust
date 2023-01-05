use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;
use super::list_node::ListNode;

type Node<K, V> = Option<Rc<RefCell<ListNode<K, V>>>>;

#[derive(Default, Debug)] // 主要给结构体赋默认的初始值
pub struct LinkedList<K, V> {
    pub head: Node<K, V>,
    pub tail: Node<K, V>
}

impl<K, V> LinkedList<K, V>
    where K: Display + Hash + Eq + Clone, V: Display + Clone + Copy + PartialEq {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    pub fn push_back(&mut self, node: Node<K, V>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(t);
            }
        } else {
            self.head = node.clone();
        }
        self.tail = node;
    }

    pub fn pop_front(&mut self) -> Node<K, V> {
        if let Some(h) = self.head.take() {
            if let Some(n) = h.borrow_mut().next.take() {
                n.borrow_mut().prev = None;
                self.head = Some(n);
            } else {
                self.head = None;
                self.tail = None;
            }
            Some(h)
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, node: Node<K, V>) -> Node<K, V> {
        if let Some(n) = node {
            let prev = n.borrow_mut().prev.take();
            let next = n.borrow_mut().next.take();

            if let Some(p) = &prev {
                p.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(n) = &next {
                n.borrow_mut().prev = prev;
            } else {
                self.tail = prev;
            }

            Some(n)
        } else {
            None
        }
    }

    pub fn print_linked_list(&self) {
        if let Some(ref cur) = self.head {
            let mut cur_node = cur.clone();
            println!("key: {}, value: {}", cur_node.borrow().key, cur_node.borrow().val);
            while let Some(ref cur) = cur_node.clone().borrow().next {
                cur_node = cur.clone();
                println!("key: {}, value: {}", cur_node.borrow().key, cur_node.borrow().val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::linked_list::LinkedList;
    use crate::list_node::ListNode;

    #[test]
    fn linked_list_test() {
        let mut linked_list: LinkedList<i32, i32> = LinkedList::new();
        let node1: Option<Rc<RefCell<ListNode<i32, i32>>>> = Some(Rc::new(RefCell::new(ListNode::new(1, 1))));
        let node2: Option<Rc<RefCell<ListNode<i32, i32>>>> = Some(Rc::new(RefCell::new(ListNode::new(2, 2))));
        let node3: Option<Rc<RefCell<ListNode<i32, i32>>>> = Some(Rc::new(RefCell::new(ListNode::new(3, 3))));
        let node4: Option<Rc<RefCell<ListNode<i32, i32>>>> = Some(Rc::new(RefCell::new(ListNode::new(4, 4))));
        let node5: Option<Rc<RefCell<ListNode<i32, i32>>>> = Some(Rc::new(RefCell::new(ListNode::new(5, 5))));
        linked_list.push_back(node1.clone());
        linked_list.push_back(node2.clone());
        linked_list.push_back(node3.clone());
        linked_list.push_back(node4.clone());
        linked_list.push_back(node5.clone());
        linked_list.print_linked_list();
        assert_eq!(node1, linked_list.remove_node(node1.clone()));
        assert_eq!(node2, linked_list.pop_front());
        assert_eq!(node3, linked_list.pop_front());
        assert_eq!(node4, linked_list.pop_front());
        assert_eq!(node5, linked_list.pop_front());
    }
}