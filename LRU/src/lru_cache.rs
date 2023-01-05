use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;
use crate::linked_list::LinkedList;
use crate::list_node::ListNode;

type Node<K, V> = Rc<RefCell<ListNode<K, V>>>;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    pub capacity: i32,
    pub size: i32,
    pub map: HashMap<K, Node<K, V>>,
    pub linked_list: LinkedList<K, V>
}

impl<K, V> LRUCache<K, V>
    where K: Display + Hash + Eq + Clone, V: Display + Clone + Copy + PartialEq {
    pub fn new(cap: i32) -> Self {
        Self {
            capacity: cap,
            size: 0,
            map: HashMap::new(),
            linked_list: LinkedList::new()
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.map.get(&key) {
            let value = node.borrow().val;
            let node = self.linked_list.remove_node(Some(node.clone()));
            self.linked_list.push_back(node);
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get_mut(&key) {
            node.borrow_mut().val = value;
            let node = self.linked_list.remove_node(Some(node.clone()));
            self.linked_list.push_back(node);
        } else {
            if self.size == self.capacity {
                if let Some(node) = self.linked_list.pop_front() {
                    self.map.remove(&node.borrow().key);
                    self.size -= 1;
                }
            }
            let new_node = Rc::new(RefCell::new(ListNode::new(key.clone(), value.clone())));
            self.map.insert(key, new_node.clone());
            self.linked_list.push_back(Some(new_node));
            self.size += 1;
        }
    }
}
