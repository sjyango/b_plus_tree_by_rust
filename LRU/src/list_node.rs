use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;
use std::ptr::eq;
use std::rc::Rc;

type Node<K, V> = Option<Rc<RefCell<ListNode<K, V>>>>;

#[derive(Debug)]
pub struct ListNode<K, V> {
    pub key: K,
    pub val: V,
    pub prev: Node<K, V>,
    pub next: Node<K, V>
}

impl<K, V> ListNode<K, V>
    where K: Display + Hash + Eq + Clone, V: Clone + Copy + Display {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None
        }
    }
}

impl<K, V> PartialEq for ListNode<K, V>
    where K: Hash + Eq + Clone, V: Clone + Copy + PartialEq {
    fn eq(&self, other: &ListNode<K, V>) -> bool {
        self.key == other.key && self.val == other.val
    }

    fn ne(&self, other: &ListNode<K, V>) -> bool {
        !eq(self,other)
    }
}