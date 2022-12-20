pub mod b_plus_tree_page;
pub mod b_plus_tree_internal_page;
pub mod b_plus_tree_leaf_page;


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn b_plus_tree_page_test() {
        let s = Arc::new(Mutex::new(String::from("hello")));
        let s2 = s.clone();

        let handler = thread::spawn(move || {
            println!("s {}", s.lock().unwrap());
        });

        {
            let mut lock_guard = s2.lock().unwrap();
            *lock_guard = String::from("helllo2");
            println!("s2 {}", lock_guard);
        }

        handler.join().unwrap();
    }
}
