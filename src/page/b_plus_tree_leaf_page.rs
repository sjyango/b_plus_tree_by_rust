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
//
//
//
//
// }
