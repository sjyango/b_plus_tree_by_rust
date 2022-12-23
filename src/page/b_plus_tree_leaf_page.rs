// use std::ptr::NonNull;
// use crate::page::b_plus_tree_page::{BPlusTreePage, MappingType, Page, SizeT};
//
// #[derive(Debug)]
// pub struct BPlusTreeLeafPage<K, V>
//     where K: Default + Clone, V: Default + Clone {
//     is_root_: bool,
//     max_size_: SizeT,
//     page_data_: Vec<MappingType<K, Option<V>>>,
//     next_page_: Page<K, V>
// }
//
// impl<K, V> From<Page<K, V>> for BPlusTreeLeafPage<K, V>
//     where K: Default + Clone, V: Default + Clone {
//     fn from(page: Page<K, V>) -> Self {
//         unsafe {
//             assert!(page.is_some());
//             if let BPlusTreePage::LeafPage(leaf_page) = page.unwrap().as_ref() {
//                 Self {
//                     is_root_: leaf_page.is_root_.clone(),
//                     max_size_: leaf_page.max_size_.clone(),
//                     page_data_: leaf_page.page_data_.to_vec(),
//                     next_page_: leaf_page.next_page_.clone()
//                 }
//             } else {
//                 unreachable!();
//             }
//         }
//     }
// }
//
// // public methods
// impl<K, V> BPlusTreeLeafPage<K, V>
//     where K: Default + Clone + PartialOrd + Copy, V: Default + Clone + PartialEq + Copy {
//
//     pub fn new(is_root: bool, max_size: SizeT) -> Page<K, V> {
//         let new_page: BPlusTreeLeafPage<K, V> = Self {
//             is_root_: is_root,
//             max_size_: max_size,
//             page_data_: Vec::new(),
//             next_page_: None
//         };
//         let mut new_page = BPlusTreePage::LeafPage(new_page);
//         NonNull::new(&mut new_page as *mut BPlusTreePage<K, V>)
//     }
//
//     pub fn is_root_page(&self) -> bool {
//         self.is_root_
//     }
//
//     pub fn get_max_size(&self) -> SizeT {
//         self.max_size_
//     }
//
//     pub fn set_max_size(&mut self, max_size: SizeT) {
//         self.max_size_ = max_size
//     }
//
//     pub fn get_min_size(&self) -> SizeT {
//         if self.is_root_page() {
//             1
//         } else {
//             (self.max_size_ + 1) / 2
//         }
//     }
//
//     pub fn get_size(&self) -> SizeT {
//         self.page_data_.len()
//     }
//
//     pub fn key_at(&self, index: usize) -> K {
//         self.page_data_[index].key.clone()
//     }
//
//     pub fn set_key_at(&mut self, index: usize, key: K) {
//         self.page_data_[index].key = key
//     }
//
//     pub fn value_at(&self, index: usize) -> Option<V> {
//         self.page_data_[index].value.clone()
//     }
//
//     pub fn set_value_at(&mut self, index: usize, value: V) {
//         self.page_data_[index].value = Some(value)
//     }
//
//     pub fn key_index(&self, key: &K) -> usize {
//         let mut left = 0 as usize;
//         let mut right = self.get_size() - 1;
//         while left <= right {
//             let mid = left + (right - left) / 2;
//             if self.key_at(mid) >= *key {
//                 right = mid - 1;
//             } else {
//                 left = mid + 1;
//             }
//         }
//         let target_index = right + 1;
//         target_index
//     }
//
//     pub fn value_index(&self, value: &V) -> Option<usize> {
//         for i in 0..=self.get_size() {
//             if self.page_data_[i].value.unwrap() == *value {
//                 return Some(i);
//             }
//         }
//         None
//     }
//
//     pub fn get_next_page(&self) -> Page<K, V> {
//         self.next_page_.clone()
//     }
//
//     pub fn set_next_page(&mut self, next_page: Page<K, V>) {
//         self.next_page_ = next_page;
//     }
//
//     pub fn check_duplicated(&self, key: K) -> bool {
//         let index = self.key_index(&key);
//         index < self.get_size() && self.key_at(index) == key
//     }
//
//     pub fn insert(&mut self, key: K, value: V) -> SizeT {
//         let insert_index = self.key_index(&key); // 查找第一个>=key的下标
//
//         // key重复了
//         if self.key_at(insert_index) == key {
//             return self.get_size();
//         }
//
//         // 数组下标>=insert_index的元素整体后移1位
//         // [insert_index, size - 1] --> [insert_index + 1, size]
//         self.page_data_.insert(insert_index, MappingType {
//             key,
//             value: Some(value)
//         });
//         self.get_size()
//     }
//
//
//
//     /// 查找internal page的array中第一个>key(注意不是>=)的下标(upper_bound)，然后据其确定value
//     ///
//     /// 注意：value指向的是子树，或者说指向的是当前内部结点的下一层某个结点
//     /// 假设下标为i的子树中的所有key为subtree(value(i))，下标为i的关键字为key(i)
//     /// 那么满足 key(i-1) <= subtree(value(i)) < key(i)
//     ///
//     /// 这里手写二分查找upper_bound，速度快于for循环的顺序查找
//     /// array类型为std::pair<KeyType, ValueType>
//     /// 正常来说下标范围是[0,size-1]，但是0位置设为无效
//     /// 所以直接从1位置开始，作为下界，下标范围是[1,size-1]
//     /// assert(GetSize() >= 1); 这里总是容易出现错误
//     pub fn lookup(&self, key: &K) -> Option<V> {
//         let target_index = self.key_index(key);
//         if target_index == self.get_size() || self.key_at(target_index) != *key {
//             None
//         } else {
//             self.page_data_[target_index].value.clone()
//         }
//     }
//
//     pub fn remove_and_delete_record(&mut self, key: K) -> SizeT {
//         let target_index = self.key_index(&key);
//         if target_index == self.get_size() || self.key_at(target_index) != key {
//             self.get_size()
//         } else {
//             self.page_data_.remove(target_index);
//             self.get_size()
//         }
//     }
//
// }
