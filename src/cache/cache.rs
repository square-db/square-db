extern crate lru;
use lazy_static::lazy_static;
use std::sync:: {
  Mutex
};
use lru::LruCache;
use std::num::NonZeroUsize;

lazy_static! {
  pub static ref GLOBAL_CACHE: Mutex<Cache<String>> = Mutex::new(Cache::new(2));
}

pub struct Cache<V> {
  cache_init: LruCache<String,
  V>,
}

impl<V> Cache<V> {
  pub fn new(capacity: usize) -> Self {
    let cache_init = LruCache::new(NonZeroUsize::new(capacity).unwrap());
    Cache {
      cache_init
    }
  }

  pub fn set(&mut self, key: String, value: V) {
    self.cache_init.put(key, value);
  }

  pub fn get(&mut self, key: String) -> Option<&mut V> {
    self.cache_init.get_mut(&key)
  }
}