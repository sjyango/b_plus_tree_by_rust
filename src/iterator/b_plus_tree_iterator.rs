use crate::page::b_plus_tree_page::{Page, ValueType};

pub struct BPlusTreeIter {
    cur_page_: Page,
    index_: usize
}

impl BPlusTreeIter {
    pub fn new(cur_page: Page, index: usize) -> Self {
        BPlusTreeIter {
            cur_page_: cur_page,
            index_: index
        }
    }
}

impl Iterator for BPlusTreeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_page_.is_none() {
            return None;
        }

        let cur_page = self.cur_page_.as_ref().unwrap();
        if cur_page.borrow().get_next_page().is_none() && self.index_ == cur_page.borrow().get_size() {
            return None;
        }

        if cur_page.borrow().get_next_page().is_some() && self.index_ == cur_page.borrow().get_size() {
            let next_page = cur_page.borrow().get_next_page();
            match next_page {
                None => {
                    self.cur_page_ = None;
                }
                Some(next_page) => {
                    self.cur_page_ = Some(next_page);
                }
            }
            self.index_ = 0;
        }

        return if let ValueType::Value(value) = self.cur_page_.as_ref().unwrap().borrow().value_at(self.index_) {
            self.index_ += 1;
            value
        } else {
            None
        }
    }
}