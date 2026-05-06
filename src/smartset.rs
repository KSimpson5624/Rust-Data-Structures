#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct SmartSet<T> {
    items: HashSet<T>,
    cache: Option<Vec<T>>,
}

impl<T> SmartSet<T> {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            cache: None,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.cache = None;
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Eq + Hash> SmartSet<T> {
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
}