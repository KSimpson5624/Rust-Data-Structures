#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::hash_set::{Drain, Iter};
use std::collections::HashSet;
use std::hash::Hash;

/// SmartSet
///
/// Underlying structure is a HashSet. It keeps the O(1) performance of a HashSet, while allowing for
/// ordering on demand.
#[derive(Debug)]
pub struct SmartSet<T> {
    items: HashSet<T>,
    cache: Option<Vec<T>>,
}

impl<T> SmartSet<T> {
    /// Creates an empty SmartSet
    ///
    /// Initial capacity of an empty SmartSet is 0.
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            cache: None,
        }
    }

    /// Clears all data from a SmartSet
    ///
    ///
    /// Example:
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::new();
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    /// assert!(!set.is_empty());
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
        self.cache = None;
    }

    /// Checks if a SmartSet is empty
    ///
    /// Returns a boolean value.
    ///
    /// Example;
    /// ```
    /// use data_structures::SmartSet;
    /// let set: SmartSet<i32> = SmartSet::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Checks the length of a SmartSet
    ///
    /// Returns usize type.
    ///
    /// Example:
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::new();
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    /// assert_eq!(set.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks the capacity of a SmartSet
    ///
    /// Returns usize.<br>
    /// The capacity of a newly initialized empty SmartSet will be 0, unless initialized with
    /// `with_capacity()`
    ///
    /// Example:
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::new();
    /// assert_eq!(set.capacity(), 0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Sets a minimum capacity for a new SmartSet
    ///
    /// Example:
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::with_capacity(50);
    /// assert!(set.capacity() >= 50);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: HashSet::with_capacity(capacity),
            cache: None,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        self.items.drain()
    }

}

impl<T: Eq + Hash> SmartSet<T> {
    /// Inserts an item into the `SmartSet`
    ///
    /// Returns a bool:
    ///     - true: item did not previously exist and is added
    ///     - false: item did previously exist and was not added
    ///
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::new();
    /// assert_eq!(set.insert(1), true);
    /// assert_eq!(set.insert(1), false);
    /// ```
    pub fn insert(&mut self, item: T) -> bool
    {
        self.items.insert(item)
    }

    pub fn get<Q>(&self, item: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.items.get(item)
    }

    pub fn contains<Q>(&self, item: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.items.contains(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut set: SmartSet<i32> = SmartSet::new();

        assert_eq!(set.insert(1), true);
        assert_eq!(set.insert(2), true);
        assert_eq!(set.insert(3), true);
        assert_eq!(set.insert(1), false);
    }

    #[test]
    fn test_clear() {
        let mut set: SmartSet<i32> = SmartSet::new();
        assert!(set.is_empty());

        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(!set.is_empty());
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_get() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.get(&1), Some(&1));
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&3), None);
    }

    #[test]
    fn test_is_empty() {
        let set: SmartSet<i32> = SmartSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_len() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2);
        set.insert(3);
        assert_eq!(set.len(), 3);
        set.insert(1);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_len_with_clear() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2);
        set.clear();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_capacity() {
        let mut set: SmartSet<i32> = SmartSet::new();
        assert_eq!(set.capacity(), 0);
        set.insert(1);
        assert!(set.capacity() >= 1);
    }

    #[test]
    fn test_with_capacity() {
        let mut set: SmartSet<i32> = SmartSet::with_capacity(10);
        assert!(set.capacity() >= 10);
    }

    #[test]
    fn test_iter() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let mut iter = set.iter();
        assert!(vec![&1, &2, &3].contains(&iter.next().unwrap()));
    }

    #[test]
    fn test_drain() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert!(!set.is_empty());

        for i in set.drain() {
            vec![1, 2, 3].contains(&i);
        }
        assert!(set.is_empty());
    }
}