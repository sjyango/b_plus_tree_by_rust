mod linked_list;
mod list_node;
mod lru_cache;


#[cfg(test)]
mod tests {
    use crate::lru_cache::LRUCache;

    #[test]
    fn lru_cache_test() {
        let mut lru_cache: LRUCache<i32, i32> = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(Some(1), lru_cache.get(1));
        lru_cache.put(3, 3);
        assert_eq!(None, lru_cache.get(2));
        lru_cache.put(4, 4);
        assert_eq!(None, lru_cache.get(1));
        assert_eq!(Some(3), lru_cache.get(3));
        assert_eq!(Some(4), lru_cache.get(4));
    }
}
