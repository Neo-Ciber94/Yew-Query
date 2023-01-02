use crate::key::QueryKey;

use super::query::Query;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

/// Provides a way to store the query data.
pub trait QueryCache: Debug {
    /// Returns the cache entry with the given key.
    fn get(&self, key: &QueryKey) -> Option<&Query>;

    /// Returns a mutable reference to the cache entry with the given key.
    fn get_mut(&mut self, key: &QueryKey) -> Option<&mut Query>;

    /// Sets a cache entry with the given key.
    fn set(&mut self, key: QueryKey, entry: Query);

    /// Removes and returns the cache entry with the given key.
    fn remove(&mut self, key: &QueryKey) -> Option<Query>;

    /// Returns `true` if the given key is in the cache.
    fn has(&self, key: &QueryKey) -> bool;

    /// Removes all the cache entries.
    fn clear(&mut self);
}

impl QueryCache for HashMap<QueryKey, Query> {
    fn get(&self, key: &QueryKey) -> Option<&Query> {
        self.get(&key)
    }

    fn get_mut(&mut self, key: &QueryKey) -> Option<&mut Query> {
        self.get_mut(&key)
    }

    fn set(&mut self, key: QueryKey, entry: Query) {
        self.insert(key, entry);
    }

    fn remove(&mut self, key: &QueryKey) -> Option<Query> {
        self.remove(key)
    }

    fn has(&self, key: &QueryKey) -> bool {
        self.contains_key(key)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl QueryCache for BTreeMap<QueryKey, Query> {
    fn get(&self, key: &QueryKey) -> Option<&Query> {
        self.get(&key)
    }

    fn get_mut(&mut self, key: &QueryKey) -> Option<&mut Query> {
        self.get_mut(&key)
    }

    fn set(&mut self, key: QueryKey, entry: Query) {
        self.insert(key, entry);
    }

    fn remove(&mut self, key: &QueryKey) -> Option<Query> {
        self.remove(key)
    }

    fn has(&self, key: &QueryKey) -> bool {
        self.contains_key(key)
    }

    fn clear(&mut self) {
        self.clear()
    }
}
