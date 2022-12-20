use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Write;
use std::mem;
use crate::iterator::b_plus_tree_iterator::BPlusTreeIter;
use crate::page::b_plus_tree_page::{BPlusTreePage, Page, RcPage, SizeT, ValueType};
use crate::page::b_plus_tree_page::BPlusTreePageType::{InternalPage, InvalidIndexPage, LeafPage};

pub enum Operation {
    FIND,
    INSERT,
    UPDATE,
    DELETE
}

pub struct BPlusTree {
    index_name_: String,
    internal_max_size_: SizeT,
    leaf_max_size_: SizeT,
    root_page_: Page
}

impl Debug for BPlusTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


// public methods
impl BPlusTree {
    pub fn new(index_name: String, internal_max_size: SizeT, leaf_max_size: SizeT) -> Self {
        Self {
            index_name_: index_name,
            internal_max_size_: internal_max_size,
            leaf_max_size_: leaf_max_size,
            root_page_: None
        }
    }

    pub fn iter(&self) -> BPlusTreeIter {
        let left_most_leaf_page = self.find_leaf_page(0, Operation::FIND, true, false);
        if left_most_leaf_page.is_none() {
            return BPlusTreeIter::new(None, 0);
        }
        BPlusTreeIter::new(left_most_leaf_page, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.root_page_.is_none()
    }

    pub fn insert(&mut self, key: i32, value: i32) -> bool {
        if self.is_empty() {
            self.create_new_tree(key, value);
            return true;
        }

        return self.insert_into_leaf(key, value);
    }

    pub fn get_value(&self, key: i32) -> Option<i32> {
        let leaf_page = self.find_leaf_page(key, Operation::FIND, false, false);
        if leaf_page.is_none() {
            return None;
        }
        let value = leaf_page.unwrap().borrow().lookup(key);
        match value {
            ValueType::Page(_) => { None }
            ValueType::Value(v) => { v }
        }
    }

    pub fn remove(&mut self, key: i32) {
        if self.is_empty() {
            return;
        }

        let leaf_page = self.find_leaf_page(key, Operation::DELETE, false, false).unwrap();
        let old_size = leaf_page.borrow().get_size();
        let new_size = leaf_page.borrow_mut().remove_and_delete_record(key);

        if old_size == new_size {
            return;
        }

        self.coalesce_or_redistribute(leaf_page);
    }

    pub fn print(&self) {
        if self.root_page_.is_none() {
            println!("Tree is Empty!");
        } else {
            let root_page = self.root_page_.as_ref().unwrap();
            println!("{}", self.to_string(root_page.clone()));
        }
    }

    pub fn draw(&self) {
        if self.root_page_.is_none() {
            println!("Tree is Empty!");
        } else {
            let root_page = self.root_page_.as_ref().unwrap();
            let graph_str = format!("digraph G {{{}}}", self.to_graph(root_page.clone()));
            println!("{}", graph_str);

            let path = "tree.dot";
            let mut f = File::create(path).unwrap();
            f.write_all(graph_str.as_bytes()).expect("write file failed");
        }
    }
}

// private methods
impl BPlusTree {
    fn to_graph(&self, cur_page: RcPage) -> String {
        let leaf_prefix = String::from("LEAF_");
        let internal_prefix = String::from("INT_");
        let mut graph_str = String::new();

        if cur_page.borrow().is_leaf_page() {
            graph_str.push_str(format!("{}{}", leaf_prefix, cur_page.borrow().get_page_id()).as_str());
            // print node properties
            graph_str.push_str("[shape=plain color=green ");
            // print data of the node
            graph_str.push_str("label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"4\">\n");
            // print data
            graph_str.push_str(format!("<TR><TD COLSPAN=\"{}\">P={}</TD></TR>\n", cur_page.borrow().get_size(), cur_page.borrow().get_page_id()).as_str());
            graph_str.push_str(format!("<TR><TD COLSPAN=\"{}\">max_size={},min_size={}</TD></TR>\n<TR>", cur_page.borrow().get_size(), cur_page.borrow().get_max_size(), cur_page.borrow().get_min_size()).as_str());
            for i in 0..cur_page.borrow().get_size() {
                graph_str.push_str(format!("<TD>{}</TD>\n", cur_page.borrow().key_at(i)).as_str());
            }
            graph_str.push_str("</TR>");
            // print table end
            graph_str.push_str("</TABLE>>];\n");
            // print Leaf node link if there is a next page
            if cur_page.borrow().get_next_page() != None {
                graph_str.push_str(format!("{}{} -> {}{};\n{{rank=same {}{} {}{}}};\n", leaf_prefix, cur_page.borrow().get_page_id(), leaf_prefix, cur_page.borrow().get_next_page().unwrap().borrow().get_page_id(), leaf_prefix, cur_page.borrow().get_page_id(), leaf_prefix, cur_page.borrow().get_next_page().unwrap().borrow().get_page_id()).as_str());
            }

            // print parent links if there is a parent
            if cur_page.borrow().get_parent_page() != None {
                graph_str.push_str(format!("{}{}:p{} -> {}{};\n", internal_prefix, cur_page.borrow().get_parent_page().unwrap().borrow().get_page_id(), cur_page.borrow().get_page_id(), leaf_prefix, cur_page.borrow().get_page_id()).as_str());
            }
        } else if cur_page.borrow().is_internal_page() {
            graph_str.push_str(format!("{}{}[shape=plain color=pink label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"4\">\n", internal_prefix, cur_page.borrow().get_page_id()).as_str());
            graph_str.push_str(format!("<TR><TD COLSPAN=\"{}\">P=\"{}\"</TD></TR>\n", cur_page.borrow().get_size(), cur_page.borrow().get_page_id()).as_str());
            graph_str.push_str(format!("<TR><TD COLSPAN=\"{}\">max_size={},min_size={}</TD></TR>\n<TR>", cur_page.borrow().get_size(), cur_page.borrow().get_max_size(), cur_page.borrow().get_min_size()).as_str());

            for i in 0..cur_page.borrow().get_size() {
                let value;
                match cur_page.borrow().value_at(i) {
                    ValueType::Page(page) => {
                        value = page.unwrap().borrow().get_page_id();
                    }
                    ValueType::Value(_) => {
                        unreachable!();
                    }
                }
                graph_str.push_str(format!("<TD PORT=\"p{}\">", value).as_str());

                if i > 0 {
                    graph_str.push_str(cur_page.borrow().key_at(i).to_string().as_str());
                } else {
                    graph_str.push_str(" ");
                }

                graph_str.push_str("</TD>\n");
            }

            graph_str.push_str("</TR></TABLE>>];\n");

            // print Parent link
            if cur_page.borrow().get_parent_page() != None {
                graph_str.push_str(format!("{}{}:p{} -> {}{};\n", internal_prefix, cur_page.borrow().get_parent_page().unwrap().borrow().get_page_id(), cur_page.borrow().get_page_id(), internal_prefix, cur_page.borrow().get_page_id()).as_str());
            }

            // print leaves
            for i in 0..cur_page.borrow().get_size() {
                let child_page;
                match cur_page.borrow().value_at(i) {
                    ValueType::Page(page) => {
                        child_page = page.unwrap();
                    }
                    ValueType::Value(_) => {
                        unreachable!();
                    }
                }
                graph_str.push_str(self.to_graph(child_page.clone()).as_str());

                if i > 0 {
                    let sibling_page;
                    match cur_page.borrow().value_at(i) {
                        ValueType::Page(page) => {
                            sibling_page = page.unwrap();
                        }
                        ValueType::Value(_) => {
                            unreachable!();
                        }
                    }

                    if !sibling_page.borrow().is_leaf_page() && !child_page.borrow().is_leaf_page() {
                        graph_str.push_str(format!("{{rank=same {}{} {}{}}};\n", internal_prefix, sibling_page.borrow().get_page_id(), internal_prefix, child_page.borrow().get_page_id()).as_str());
                    }
                }
            }
        }

        graph_str
    }

    fn to_string(&self, cur_page: RcPage) -> String {
        let page_id = cur_page.borrow().get_page_id();
        let parent_page_id;
        let next_page_id;

        if cur_page.borrow().get_parent_page().is_some() {
            parent_page_id = cur_page.borrow().get_parent_page().as_ref().unwrap().borrow().get_page_id().to_string();
        } else {
            parent_page_id = String::from("None");
        }

        if cur_page.borrow().get_next_page().is_some() {
            next_page_id = cur_page.borrow().get_next_page().as_ref().unwrap().borrow().get_page_id().to_string();
        } else {
            next_page_id = String::from("None");
        }

        if cur_page.borrow().is_leaf_page() {
            let mut leaf_str = format!("Leaf Page: {} Parent: {} Next: {}\n", page_id, parent_page_id, next_page_id);

            for i in 0..cur_page.borrow().get_size() {
                let key = cur_page.borrow().key_at(i);
                leaf_str.push_str(format!("{}, ", key).as_str());
            }

            leaf_str.push_str("\n\n");

            leaf_str
        } else if cur_page.borrow().is_internal_page() {
            let mut internal_str = format!("Internal Page: {} Parent: {} Next: {}\n", page_id, parent_page_id, next_page_id);

            for i in 0..cur_page.borrow().get_size() {
                let key = cur_page.borrow().key_at(i);

                match cur_page.borrow().value_at(i) {
                    ValueType::Page(child_page) => {
                        let value = child_page.as_ref().unwrap().borrow().get_page_id();
                        internal_str.push_str(format!("{} : {}, ", key, value).as_str());
                    }
                    ValueType::Value(value) => {
                        let value = match value {
                            None => {
                                String::from("-")
                            }
                            Some(v) => {
                                v.to_string()
                            }
                        };
                        internal_str.push_str(format!("{} : {}, ", key, value).as_str());
                    }
                }
            }

            internal_str.push_str("\n\n");

            for i in 0..cur_page.borrow().get_size() {
                if let ValueType::Page(child_page) = cur_page.borrow().value_at(i) {
                    let child_page = child_page.unwrap();
                    internal_str.push_str(self.to_string(child_page).as_str());
                } else {
                    unreachable!();
                }
            }

            internal_str
        } else {
            String::new()
        }
    }

    fn find_leaf_page(&self, key: i32, operation: Operation, left_most: bool, right_most: bool) -> Page {
        if self.root_page_.is_none() {
            return None
        }

        let mut cur_page = self.root_page_.clone().unwrap();

        while cur_page.borrow().is_internal_page() {
            let child_page;
            if left_most {
                child_page = cur_page.borrow().value_at(0);
            } else if right_most {
                child_page = cur_page.borrow().value_at(cur_page.borrow().get_size() - 1);
            } else {
                child_page = cur_page.borrow().lookup(key);
            }
            assert!(child_page != ValueType::Page(None));

            match operation {
                Operation::FIND => {
                    // TODO
                }
                Operation::INSERT => {
                    // TODO
                }
                Operation::UPDATE => {
                    // TODO
                }
                Operation::DELETE => {
                    // TODO
                }
            }

            if let ValueType::Page(page) = child_page {
                cur_page = page.unwrap();
            } else {
                unreachable!();
            }
        }

        Some(cur_page)
    }

    fn create_new_tree(&mut self, key: i32, value: i32) {
        let new_root = BPlusTreePage::new(LeafPage, self.leaf_max_size_, None);
        new_root.borrow_mut().insert(key, value);
        self.root_page_ = Some(new_root);
    }

    fn insert_into_leaf(&mut self, key: i32, value: i32) -> bool {
        let leaf_page = self.find_leaf_page(key, Operation::INSERT, false, false);
        if leaf_page.is_none() {
            return false;
        }

        let leaf_page = leaf_page.unwrap();
        let old_size = leaf_page.borrow().get_size();
        let new_size = leaf_page.borrow_mut().insert(key, value);

        if old_size == new_size {
            return false;
        }

        if leaf_page.borrow().is_internal_page() {
            if new_size <= self.leaf_max_size_ {
                return true;
            }
        } else if leaf_page.borrow().is_leaf_page() {
            if new_size < self.leaf_max_size_ {
                return true;
            }
        }

        let sibling_leaf_page = self.split(leaf_page.clone()).unwrap();
        sibling_leaf_page.borrow_mut().set_next_page(leaf_page.borrow().get_next_page());
        leaf_page.borrow_mut().set_next_page(Some(sibling_leaf_page.clone()));
        let middle_key = sibling_leaf_page.borrow().key_at(0);

        self.insert_into_parent(leaf_page.clone(), middle_key, sibling_leaf_page.clone());

        true
    }

    fn split(&mut self, cur_page: RcPage) -> Page {
        if cur_page.borrow().is_internal_page() {
            let new_page = BPlusTreePage::new(InternalPage, self.internal_max_size_, cur_page.borrow().get_parent_page());
            cur_page.borrow_mut().move_half_to(new_page.clone());
            return Some(new_page);
        } else if cur_page.borrow().is_leaf_page() {
            let new_page = BPlusTreePage::new(LeafPage, self.leaf_max_size_, cur_page.borrow().get_parent_page());
            cur_page.borrow_mut().move_half_to(new_page.clone());
            return Some(new_page);
        }
        None
    }

    fn insert_into_parent(&mut self, old_page: RcPage, middle_key: i32, new_page: RcPage) {
        if old_page.borrow().is_root_page() {
            let new_root = BPlusTreePage::new(InternalPage, self.internal_max_size_, None);
            new_root.borrow_mut().create_new_root(old_page.clone(), middle_key, new_page.clone());
            old_page.borrow_mut().set_parent_page(Some(new_root.clone()));
            new_page.borrow_mut().set_parent_page(Some(new_root.clone()));
            self.root_page_ = Some(new_root);
            return;
        }

        let parent_page = old_page.borrow().get_parent_page().unwrap();
        let new_size = parent_page.borrow_mut().insert_node_after(old_page.clone(), middle_key, new_page.clone());
        // TODO
        new_page.borrow_mut().set_parent_page(Some(parent_page.clone()));

        // -1是去掉下标为0的item
        if new_size - 1 < self.internal_max_size_ {
            return;
        }

        let new_parent_sibling_node = self.split(parent_page.clone()).unwrap();
        let middle_key = new_parent_sibling_node.borrow().key_at(0);
        self.insert_into_parent(parent_page.clone(), middle_key, new_parent_sibling_node.clone());
    }

    fn coalesce(&mut self, neighbor_page: &mut RcPage, cur_page: &mut RcPage, parent_page: RcPage, index: usize) -> bool {
        let mut key_index = index;

        if index == 0 {
            key_index = 1;
            mem::swap(cur_page, neighbor_page);
        }

        let middle_key = parent_page.borrow().key_at(key_index);
        cur_page.borrow_mut().move_all_to((*neighbor_page).clone(), middle_key);
        (*neighbor_page).borrow_mut().set_next_page(cur_page.borrow().get_next_page());

        parent_page.borrow_mut().remove(key_index);
        return self.coalesce_or_redistribute(parent_page.clone());
    }

    fn redistribute(&mut self, neighbor_page: RcPage, cur_page: RcPage, parent_page: RcPage, index: usize) {
        if cur_page.borrow().is_leaf_page() {
            if index == 0 {
                neighbor_page.borrow_mut().move_first_to_end_of(cur_page, 0);
                parent_page.borrow_mut().set_key_at(1, neighbor_page.borrow().key_at(0));
            } else {
                neighbor_page.borrow_mut().move_last_to_front_of(cur_page, 0);
                parent_page.borrow_mut().set_key_at(index, neighbor_page.borrow().key_at(0));
            }
        } else if cur_page.borrow().is_internal_page() {
            if index == 0 {
                neighbor_page.borrow_mut().move_first_to_end_of(cur_page, parent_page.borrow().key_at(1));
                parent_page.borrow_mut().set_key_at(1, neighbor_page.borrow().key_at(0));
            } else {
                neighbor_page.borrow_mut().move_last_to_front_of(cur_page, parent_page.borrow().key_at(index));
                parent_page.borrow_mut().set_key_at(index, neighbor_page.borrow().key_at(0));
            }
        }
    }

    fn coalesce_or_redistribute(&mut self, cur_page: RcPage) -> bool {
        if cur_page.borrow().is_root_page() {
            return self.adjust_root(cur_page);
        }

        let cur_size = cur_page.borrow().get_size();
        let min_size = cur_page.borrow().get_min_size();

        if cur_size >= min_size {
            return false;
        }

        let parent_page = cur_page.borrow().get_parent_page().unwrap();
        let cur_page_index = parent_page.borrow().value_index( ValueType::Page(Some(cur_page.clone()))).unwrap();

        let mut sibling_page: RcPage = BPlusTreePage::new(InvalidIndexPage, 0, None);

        if cur_page_index == 0 {
            if let ValueType::Page(sibling) = parent_page.borrow().value_at(1) {
                sibling_page = sibling.unwrap().clone();
            }
        } else {
            if let ValueType::Page(sibling) = parent_page.borrow().value_at(cur_page_index - 1) {
                sibling_page = sibling.unwrap().clone();
            }
        }

        let coalesce_size = cur_page.borrow().get_size() + sibling_page.borrow().get_size();
        let max_size = cur_page.borrow().get_max_size();

        if coalesce_size > max_size {
            self.redistribute(sibling_page.clone(), cur_page, parent_page.clone(), cur_page_index);
            return false;
        }

        let _ = self.coalesce(&mut sibling_page.clone(), &mut cur_page.clone(), parent_page.clone(), cur_page_index);
        return true;
    }

    fn adjust_root(&mut self, old_root_page: RcPage) -> bool {
        if old_root_page.borrow().is_internal_page() && old_root_page.borrow().get_size() == 1 {
            if let ValueType::Page(only_child_page) = old_root_page.borrow().value_at(0) {
                let only_child_page = only_child_page.unwrap();
                only_child_page.borrow_mut().set_parent_page(None);
                self.root_page_ = Some(only_child_page.clone());
                return true;
            }
        }
        old_root_page.borrow().is_leaf_page() && old_root_page.borrow().get_size() == 0
    }
}