// use std::ptr::NonNull;
// use crate::page::b_plus_tree_page::{BPlusTreePage, Page};
// use super::b_plus_tree_page::{SizeT, MappingType};
//
//
// #[derive(Debug)]
// pub struct BPlusTreeInternalPage<K, V>
//     where K: Default + Clone, V: Default + Clone {
//     is_root_: bool,
//     max_size_: SizeT,
//     page_data_: Vec<MappingType<K, Page<K, V>>>
// }
//
//
//
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
//             2
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
//     pub fn value_at(&self, index: usize) -> Page<K, V> {
//         self.page_data_[index].value.clone()
//     }
//
//     pub fn set_value_at(&mut self, index: usize, value: Page<K, V>) {
//         self.page_data_[index].value = value
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
//     pub fn value_index(&self, value: &Page<K, V>) -> Option<usize> {
//         for i in 0..=self.get_size() {
//             if self.page_data_[i].value == *value {
//                 return Some(i);
//             }
//         }
//         None
//     }
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
//     pub fn lookup(&self, key: &K) -> Page<K, V> {
//         let mut left = 1;
//         let mut right = self.get_size() - 1;
//
//         while left <= right {
//             let mid = left + (right - left) / 2;
//             if self.key_at(mid) > *key {
//                 right = mid - 1;
//             } else {
//                 left = mid + 1;
//             }
//         }
//
//         let target_index = left;
//         assert!(target_index - 1 >= 0);
//         self.value_at(target_index - 1)
//     }
//
//     /// Populate new root page with old_value + new_key & new_value
//     /// When the insertion cause overflow from leaf page all the way upto the root
//     /// page, you should create a new root page and populate its elements.
//     /// NOTE: This method is only called within InsertIntoParent()(b_plus_tree.cpp)
//     pub fn create_new_root(&mut self, old_value: Page<K, V>, new_key: K, new_value: Page<K, V>) {
//         self.page_data_.resize_with(2, || { MappingType::new() });
//         self.set_key_at(1, new_key);
//         self.set_value_at(0, old_value);
//         self.set_value_at(1, new_value);
//         self.is_root_ = true;
//         assert_eq!(self.get_size(), 2);
//     }
//
//     /// Insert new_key & new_value pair right after the pair with its value == old_value
//     pub fn insert_node_after(&mut self, old_value: Page<K, V>, new_key: K, new_value: Page<K, V>) -> SizeT {
//         if let Some(old_value_index) = self.value_index(&old_value) {
//             self.page_data_.insert(old_value_index + 1, MappingType { key: new_key, value: new_value });
//             self.get_size()
//         } else {
//             // TODO
//             self.get_size()
//         }
//     }
//
//     /// this page是old_node，recipient page是new_node
//     /// old_node的右半部分array复制给new_node
//     /// 并且，将new_node（原old_node的右半部分）的所有孩子结点的父指针更新为指向new_node
//     pub unsafe fn move_half_to(&mut self, recipient: Page<K, V>) {
//         let start_index = self.get_min_size();
//         let pre_size = self.get_size();
//         let move_num = pre_size - start_index;
//         if let BPlusTreePage::InternalPage(internal_page) = recipient.unwrap().as_mut() {
//             internal_page.page_data_.append(&mut self.page_data_.split_off(start_index));
//             assert_eq!(pre_size - move_num, self.get_size());
//         }
//     }
//

