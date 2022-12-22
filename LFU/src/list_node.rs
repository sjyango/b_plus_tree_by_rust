use std::cell::RefCell;
use std::hash::Hash;
use std::ptr::eq;
use std::rc::Rc;

type Node<K, V> = Option<Rc<RefCell<ListNode<K, V>>>>;

#[derive(Debug)]
pub struct ListNode<K, V> {
    pub key: K,
    pub value: V,
    pub freq: i32,
    pub prev: Node<K, V>,
    pub next: Node<K, V>
}

impl<K: Hash + Eq + Clone, V: Clone + Copy> ListNode<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            freq: 1,
            prev: None,
            next: None
        }
    }
}

impl<K, V> PartialEq for ListNode<K, V>
    where K: Hash + Eq + Clone, V: Clone + Copy + PartialEq {
    fn eq(&self, other: &ListNode<K, V>) -> bool {
        self.key == other.key && self.value == other.value
    }

    fn ne(&self, other: &ListNode<K, V>) -> bool {
        !eq(self,other)
    }
}