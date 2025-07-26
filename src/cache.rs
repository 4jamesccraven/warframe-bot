use std::collections::{HashSet, VecDeque};
use std::fs::{self, File};
use std::hash::Hash;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use bincode::serde::{decode_from_std_read, encode_into_std_write};
use dirs::cache_dir;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait Cacheable: Eq + Hash + Clone {}
impl<T: Eq + Hash + Clone> Cacheable for T {}

#[derive(Debug, Clone, Serialize)]
pub struct SeenCache<T, const CACHE_SIZE: usize>
where
    T: Cacheable,
{
    cache_name: String,
    queue: VecDeque<T>,
    #[serde(skip)]
    set: HashSet<T>,
}

impl<T, const CACHE_SIZE: usize> SeenCache<T, CACHE_SIZE>
where
    T: Eq + Hash + Clone,
{
    /// Adds a value to the cache, dropping the oldest value when exceeding the cache size.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned,
    ///   and the set is not modified: original value is not replaced,
    ///   and the value passed as argument is dropped.
    pub fn insert(&mut self, value: T) -> bool {
        if self.contains(&value) {
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

    #[allow(unused)]
    /// Returns the length of the cache.
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
            .filter(|&value| self.insert(value.clone()))
            .cloned()
            .collect()
    }
}

impl<T, const N: usize> SeenCache<T, N>
where
    T: Cacheable + DeserializeOwned + Serialize,
{
    /// Load the cache from its default location, or provide an empty one.
    pub fn new(cache_name: &str) -> Self {
        Self::from_cache(cache_name).unwrap_or(Self::default())
    }

    /// Get the path to the cache directory.
    fn cache_path(cache_name: &str) -> Option<PathBuf> {
        let path = cache_dir()?.join("wf_bot");
        fs::create_dir_all(&path).ok()?;

        Some(path.join(format!("cache_{cache_name}.bin")))
    }

    /// Attempt to load a binary dump of the cache from the default location.
    fn from_cache(cache_name: &str) -> Option<Self> {
        let cache = Self::cache_path(cache_name)?;
        let mut file_handle = File::open(cache).ok()?;

        let cfg = bincode::config::standard();
        let cache: Self = decode_from_std_read(&mut file_handle, cfg).ok()?;

        Some(cache)
    }

    /// Dump the cache to the default location.
    pub fn dump(&self) -> Result<()> {
        let cache = Self::cache_path(&self.cache_name)
            .ok_or_else(|| anyhow!("could not get cache path, skipping cache dump"))?;
        let file_handle = File::create(cache)?;
        let mut writer = BufWriter::new(file_handle);

        let cfg = bincode::config::standard();
        encode_into_std_write(self, &mut writer, cfg)?;

        Ok(())
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
            cache_name: "DEFAULT".to_string(),
        }
    }
}

impl<T, const N: usize> From<&[T]> for SeenCache<T, N>
where
    T: Eq + Hash + Clone,
{
    fn from(value: &[T]) -> Self {
        let mut cache = Self::default();

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
        #[derive(Deserialize)]
        struct SeenCacheHelper<T> {
            cache_name: String,
            queue: VecDeque<T>,
        }

        let helper = SeenCacheHelper::deserialize(deserializer)?;
        let mut set = HashSet::with_capacity(helper.queue.len());

        helper.queue.iter().for_each(|i: &T| {
            set.insert(i.clone());
        });

        Ok(SeenCache {
            queue: helper.queue,
            cache_name: helper.cache_name,
            set,
        })
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
        let serialized = bincode::serde::encode_to_vec(&cache, cfg).unwrap();
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
        let mut cache: SeenCache<usize, 3> = SeenCache::default();
        assert_eq!(cache.len(), 0);

        cache.insert(4);
        assert_eq!(cache.len(), 1);
    }
}
