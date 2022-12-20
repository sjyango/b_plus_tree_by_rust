pub mod b_plus_tree_iterator;


#[cfg(test)]
mod tests {
    use crate::index::b_plus_tree::BPlusTree;

    #[test]
    fn b_plus_tree_iterator_test() {
        let mut tree = BPlusTree::new(String::from("tree1"), 3, 3);

        for i in 0..=10 {
            tree.insert(i, i);
        }

        tree.draw();

        // tree.remove(0);
        // tree.remove(1);
        // tree.remove(2);
        // tree.remove(3);
        // tree.remove(4);
        // tree.remove(7);

        // tree.print();

        // for item in tree.iter() {
        //     print!("{} ", item);
        // }
    }
}