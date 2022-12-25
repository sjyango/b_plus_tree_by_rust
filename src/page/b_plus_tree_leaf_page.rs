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
//
//
//
// }
