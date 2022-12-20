use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::linked_list::LinkedList;
use crate::list_node::ListNode;

type Node<K, V> = Rc<RefCell<ListNode<K, V>>>;
type List<K, V> = Rc<RefCell<LinkedList<K, V>>>;

#[derive(Default, Debug)]
pub struct LFUCache<K, V> {
    pub capacity: i32,
    pub size: i32,
    pub min_freq: i32,
    pub key_map: HashMap<K, Node<K, V>>,
    pub freq_map: HashMap<i32, List<K, V>>
}

impl<K, V> LFUCache<K, V>
    where K: Hash + Eq + Clone, V: Clone + Copy + PartialEq {
    pub fn new(cap: i32) -> Self {
        Self {
            capacity: cap,
            size: 0,
            min_freq: 0,
            key_map: HashMap::new(),
            freq_map: HashMap::new()
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }

        if let Some(node) = self.key_map.get(&key) {
            // key exists, get node from key map and return value obtaining from the node
            //
            let value = node.borrow().value;
            self.update(node.clone());
            Some(value)
        } else {
            // key doesn't exist, just return None
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

        if let Some(node) = self.key_map.get(&key) {
            node.borrow_mut().value = value;
            self.update(node.clone());
        } else {
            if self.size == self.capacity {
                let node = if let Some(list) = self.freq_map.get_mut(&self.min_freq) {
                    list.borrow_mut().pop_front()
                } else {
                    None
                };
                if let Some(n) = node {
                    self.key_map.remove(&n.borrow().key);
                }
            } else {
                self.size += 1;
            }
            let new_node = Rc::new(RefCell::new(ListNode::new(key.clone(), value)));
            self.min_freq = 1;
            self.key_map.insert(key, new_node.clone());
            self.freq_map.entry(1).or_insert(Rc::new(RefCell::new(LinkedList::new()))).borrow_mut().push_back(Some(new_node));
        }
    }

    pub fn update(&mut self, node: Node<K, V>) {
        let freq = node.borrow().freq;
        node.borrow_mut().freq += 1;

        // Gets old list with old frequency, and remove current node from old list
        if let Some(list) = self.freq_map.get_mut(&freq) {
            list.borrow_mut().remove_node(Some(node.clone()));
        }

        // Gets new list with new frequency, and insert current node into new list
        self.freq_map.entry(freq + 1).or_insert(Rc::new(RefCell::new(LinkedList::new()))).borrow_mut().push_back(Some(node));

        if let Some(list) = self.freq_map.get(&freq) {
            if list.borrow().is_empty() && freq == self.min_freq {
                self.min_freq += 1;
                self.freq_map.remove(&freq);
            }
        }
    }
}
