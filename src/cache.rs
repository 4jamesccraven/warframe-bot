use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SeenCache<T, const CACHE_SIZE: usize>
where
    T: Eq + Hash + Clone,
{
    queue: VecDeque<T>,
    #[serde(skip)]
    set: HashSet<T>,
}

impl<T, const CACHE_SIZE: usize> SeenCache<T, CACHE_SIZE>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a value to the cache, dropping the oldest value when exceeding the cache size.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned,
    ///   and the set is not modified: original value is not replaced,
    ///   and the value passed as argument is dropped.
    pub fn insert(&mut self, value: T) -> bool {
        if self.set.contains(&value) {
            return false;
        }

        if self.queue.len() == CACHE_SIZE {
            if let Some(oldest) = self.queue.pop_front() {
                self.set.remove(&oldest);
            }
        }
        self.set.insert(value.clone());
        self.queue.push_back(value);
        true
    }

    /// Returns the length of the cache
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns `true` if the cache contains the value
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    /// Updates the cache with a list of items, ignoring previously seen values and caching new
    /// ones.
    ///
    /// Returns the list of newly added item. That is, unseen items.
    pub fn difference(&mut self, values: &[T]) -> Vec<T> {
        values
            .iter()
            .cloned()
            .filter(|value| self.insert(value.clone()))
            .collect()
    }
}

impl<T, const CACHE_SIZE: usize> Default for SeenCache<T, CACHE_SIZE>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            queue: VecDeque::with_capacity(CACHE_SIZE),
            set: HashSet::with_capacity(CACHE_SIZE),
        }
    }
}

impl<T, const N: usize> From<&[T]> for SeenCache<T, N>
where
    T: Eq + Hash + Clone,
{
    fn from(value: &[T]) -> Self {
        let mut cache = Self::new();

        value.iter().cloned().for_each(|item| {
            cache.insert(item);
        });

        cache
    }
}

impl<T, const N: usize> PartialEq for SeenCache<T, N>
where
    T: Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.queue == other.queue
    }
}

impl<T, const N: usize> Eq for SeenCache<T, N> where T: Eq + Hash + Clone {}

impl<'de, T, const CACHE_SIZE: usize> Deserialize<'de> for SeenCache<T, CACHE_SIZE>
where
    T: Eq + Hash + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let queue = VecDeque::deserialize(deserializer)?;
        let mut set = HashSet::with_capacity(queue.len());

        queue.iter().for_each(|i: &T| {
            set.insert(i.clone());
        });

        Ok(SeenCache { queue, set })
    }
}

#[cfg(test)]
mod cache_test {
    use super::*;

    #[test]
    fn deserialize() {
        use bincode;

        let cache: SeenCache<usize, 3> = SeenCache::from(&vec![1, 2, 3][..]);

        let cfg = bincode::config::standard();
        let serialized = bincode::serde::encode_to_vec(&cache, cfg.clone()).unwrap();
        let deserialized = bincode::serde::decode_from_slice(&serialized, cfg).unwrap();

        assert_eq!(cache, deserialized.0);
    }

    #[test]
    fn contains() {
        let cache: SeenCache<usize, 3> = SeenCache::from(&vec![1, 2, 3][..]);

        assert!(cache.contains(&1));
        assert!(!cache.contains(&47));
    }

    #[test]
    fn insert() {
        let mut cache: SeenCache<usize, 3> = SeenCache::new();
        assert_eq!(cache.len(), 0);

        cache.insert(4);
        assert_eq!(cache.len(), 1);
    }
}
