use std::cell::RefCell;
use std::rc::Rc;

use std::sync::atomic::{AtomicUsize, Ordering};

pub type Page = Option<Rc<RefCell<BPlusTreePage>>>;
pub type RcPage = Rc<RefCell<BPlusTreePage>>;
pub type SizeT = usize;

static PAGE_ID_ATOMIC: AtomicUsize  = AtomicUsize::new(0);


#[derive(PartialEq)]
pub enum BPlusTreePageType {
    InvalidIndexPage,
    InternalPage,
    LeafPage
}

pub struct BPlusTreePage {
    page_id_: usize,
    page_type_: BPlusTreePageType,
    max_size_: SizeT,
    page_data_: Vec<MappingType>,
    parent_page_: Page,
    next_page_: Page
}

impl PartialEq for BPlusTreePage {
    fn eq(&self, other: &Self) -> bool {
        self.page_id_ == other.page_id_
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Clone)]
pub enum ValueType {
    Page(Page), // 指代page
    Value(Option<i32>) // 指代真正的value
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Value(None)
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            ValueType::Page(page) => {
                match other {
                    ValueType::Page(other_page) => {
                        if page.is_none() && other_page.is_none() {
                            return true;
                        }

                        if (page.is_none() && other_page.is_some()) || (page.is_some() && other_page.is_none()) {
                            return false;
                        }
                        (*page).as_ref().unwrap() == (*other_page).as_ref().unwrap()
                    }
                    ValueType::Value(_) => {
                        false
                    }
                }
            }
            ValueType::Value(value) => {
                match other {
                    ValueType::Page(_) => {
                        false
                    }
                    ValueType::Value(other_value) => {
                        if value.is_none() && other_value.is_none() {
                            return true;
                        }

                        if (value.is_none() && other_value.is_some()) || (value.is_some() && other_value.is_none()) {
                            return false;
                        }
                        *value == *other_value
                    }
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


#[derive(Clone, Default, PartialEq)]
pub struct MappingType {
    pub key: i32,
    pub value: ValueType,
}

impl BPlusTreePage {
    pub fn new(page_type: BPlusTreePageType, max_size: SizeT, parent_page: Page) -> RcPage {
        let new_page: BPlusTreePage = Self {
            page_id_: PAGE_ID_ATOMIC.fetch_add(1, Ordering::Relaxed),
            page_type_: page_type,
            max_size_: max_size,
            page_data_: Vec::new(),
            parent_page_: parent_page,
            next_page_: None
        };
        Rc::new(RefCell::new(new_page))
    }

    pub fn get_page_id(&self) -> usize {
        self.page_id_
    }

    pub fn is_root_page(&self) -> bool {
        self.parent_page_.is_none()
    }

    pub fn is_internal_page(&self) -> bool {
        self.page_type_ == BPlusTreePageType::InternalPage
    }

    pub fn is_leaf_page(&self) -> bool {
        self.page_type_ == BPlusTreePageType::LeafPage
    }

    pub fn get_max_size(&self) -> SizeT {
        self.max_size_
    }

    pub fn set_max_size(&mut self, max_size: SizeT) {
        self.max_size_ = max_size
    }

    /// m阶B+树定义
    ///
    /// B+树是B树的一种变形形式，m阶B+树满足以下条件：
    ///
    /// (1) 每个结点至多有m个孩子。
    ///     a. 意味着对于内部节点，最多只有m-1个关键字
    ///     b. 意味着对于叶子节点，最多可以有m个关键字
    ///
    /// (2) 除根节点和叶结点外，每个结点至少有 (m + 1) / 2 个孩子，也就是说至少有 (m - 1) / 2 个关键字。
    ///
    /// (3) 如果根节点不为空，根结点至少有两个孩子。
    ///
    /// (4) 所有叶子结点增加一个链指针，所有关键字都在叶子结点出现。
    pub fn get_min_size(&self) -> SizeT {
        return if self.is_root_page() {
            if self.is_internal_page() {
                2
            } else if self.is_leaf_page() {
                1
            } else {
                0
            }
        } else {
            if self.is_internal_page() {
                (self.max_size_ + 1) / 2
            } else if self.is_leaf_page() {
                self.max_size_ / 2
            } else {
                0
            }
        }
    }

    pub fn get_size(&self) -> SizeT {
        self.page_data_.len()
    }

    pub fn key_at(&self, index: usize) -> i32 {
        self.page_data_[index].key.clone()
    }

    pub fn set_key_at(&mut self, index: usize, key: i32) {
        self.page_data_[index].key = key
    }

    pub fn value_at(&self, index: usize) -> ValueType {
        self.page_data_[index].value.clone()
    }

    pub fn set_value_at(&mut self, index: usize, value: ValueType) {
        self.page_data_[index].value = value
    }

    pub fn key_index(&self, key: i32) -> usize {
        let mut left = 0;
        let mut right = self.get_size() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if self.key_at(mid as usize) as i32 >= key {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        let target_index = right + 1;
        target_index as usize
    }

    pub fn value_index(&self, value: ValueType) -> Option<usize> {
        for i in 0..self.get_size() {
            if self.page_data_[i].value == value {
                return Some(i);
            }
        }
        None
    }

    pub fn get_parent_page(&self) -> Page {
        self.parent_page_.clone()
    }

    pub fn set_parent_page(&mut self, parent_page: Page) {
        self.parent_page_ = parent_page;
    }

    pub fn get_next_page(&self) -> Page {
        self.next_page_.clone()
    }

    pub fn set_next_page(&mut self, next_page: Page) {
        self.next_page_ = next_page;
    }

    pub fn lookup(&self, key: i32) -> ValueType {
        if self.is_internal_page() {
            let mut left = 1;
            let mut right = self.get_size() - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if self.key_at(mid) > key {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            let target_index = left;
            assert!(target_index - 1 >= 0);
            self.value_at(target_index - 1)
        } else if self.is_leaf_page() {
            let target_index = self.key_index(key);
            if target_index == self.get_size() || self.key_at(target_index) != key {
                ValueType::Value(None)
            } else {
                self.page_data_[target_index].value.clone()
            }
        } else {
            ValueType::Value(None)
        }
    }

    pub fn create_new_root(&mut self, old_page: RcPage, new_key: i32, new_page: RcPage) {
        let item1 = MappingType {
            key: Default::default(),
            value: ValueType::Page(Some(old_page))
        };
        let item2 = MappingType {
            key: new_key,
            value: ValueType::Page(Some(new_page))
        };
        self.page_data_.push(item1);
        self.page_data_.push(item2);
        assert_eq!(self.get_size(), 2);
    }

    // only can be invoked by internal page
    pub fn insert_node_after(&mut self, old_value: RcPage, new_key: i32, new_value: RcPage) -> SizeT {
        if let Some(old_value_index) = self.value_index(ValueType::Page(Some(old_value))) {
            self.page_data_.insert(old_value_index + 1, MappingType { key: new_key, value: ValueType::Page(Some(new_value)) });
            self.get_size()
        } else {
            // TODO
            self.get_size()
        }
    }

    pub fn insert(&mut self, key: i32, value: i32) -> SizeT {
        let insert_index = self.key_index(key); // 查找第一个>=key的下标

        // key重复了
        if insert_index < self.get_size() && self.key_at(insert_index) == key {
            return self.get_size();
        }

        // [insert_index, size - 1] --> [insert_index + 1, size]
        self.page_data_.insert(insert_index, MappingType {
            key,
            value: ValueType::Value(Some(value))
        });
        self.get_size()
    }

    pub fn remove(&mut self, index: usize) {
        self.page_data_.remove(index);
    }

    pub fn remove_and_return_only_child(&mut self) -> Page {
        let value = self.value_at(0);
        self.page_data_.clear();
        if let ValueType::Page(page) = value {
            page
        } else {
            None
        }
    }

    pub fn remove_and_delete_record(&mut self, key: i32) -> SizeT {
        let target_index = self.key_index(key);
        if target_index != self.get_size() && self.key_at(target_index) == key {
            self.page_data_.remove(target_index);
        }
        self.get_size()
    }

    pub fn move_half_to(&mut self, recipient: RcPage) {
        let start_index = self.get_min_size();
        let pre_size = self.get_size();
        let move_num = pre_size - start_index;
        let mut moved_items = self.page_data_.split_off(start_index);
        for item in &mut moved_items {
            match &mut item.value {
                ValueType::Page(child_page) => {
                    (*child_page).as_ref().unwrap().borrow_mut().parent_page_ = Some(recipient.clone());
                }
                ValueType::Value(_) => {
                    // todo nothing!
                }
            }
        }
        recipient.borrow_mut().page_data_.append(&mut moved_items);
        assert_eq!(pre_size - move_num, self.get_size());
    }

    pub fn move_all_to(&mut self, recipient: RcPage, middle_key: i32) {
        if self.is_internal_page() {
            self.set_key_at(0, middle_key);
        }

        let mut moved_items = self.page_data_.to_vec();
        self.page_data_.clear();
        for item in &mut moved_items {
            match &mut item.value {
                ValueType::Page(child_page) => {
                    (*child_page).as_ref().unwrap().borrow_mut().parent_page_ = Some(recipient.clone());
                }
                ValueType::Value(_) => {
                    // todo nothing!
                }
            }
        }
        recipient.borrow_mut().page_data_.append(&mut moved_items);
    }

    pub fn move_first_to_end_of(&mut self, recipient: RcPage, middle_key: i32) {
        if self.is_internal_page() {
            self.set_key_at(0, middle_key);
        }
        assert!(self.page_data_.len() > 0);
        let mut first_item = self.page_data_.remove(0);
        if self.is_internal_page() {
            match &mut first_item.value {
                ValueType::Page(child_page) => {
                    (*child_page).as_ref().unwrap().borrow_mut().parent_page_ = Some(recipient.clone());
                }
                ValueType::Value(_) => {
                    // todo nothing!
                }
            }
        }
        recipient.borrow_mut().page_data_.push(first_item);
    }

    pub fn move_last_to_front_of(&mut self,  recipient: RcPage, middle_key: i32) {
        recipient.borrow_mut().set_key_at(0, middle_key);
        assert!(self.page_data_.len() > 0);
        let mut last_item = self.page_data_.pop().unwrap();
        if self.is_internal_page() {
            match &mut last_item.value {
                ValueType::Page(child_page) => {
                    (*child_page).as_ref().unwrap().borrow_mut().parent_page_ = Some(recipient.clone());
                }
                ValueType::Value(_) => {
                    // todo nothing!
                }
            }
        }
        recipient.borrow_mut().page_data_.insert(0, last_item);
    }
}




