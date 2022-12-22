mod lfu_cache;
mod linked_list;
mod list_node;


#[cfg(test)]
mod tests {
    use crate::lfu_cache::LFUCache;

    #[test]
    fn lfu_cache_test() {
        let mut lfu_cache: LFUCache<i32, i32> = LFUCache::new(2);
        lfu_cache.put(1, 1);
        lfu_cache.put(2, 2);
        assert_eq!(Some(1), lfu_cache.get(1));
        lfu_cache.put(3, 3);
        assert_eq!(None, lfu_cache.get(2));
        assert_eq!(Some(3), lfu_cache.get(3));
        lfu_cache.put(4, 4);
        assert_eq!(None, lfu_cache.get(1));
        assert_eq!(Some(3), lfu_cache.get(3));
        assert_eq!(Some(4), lfu_cache.get(4));
    }
}
