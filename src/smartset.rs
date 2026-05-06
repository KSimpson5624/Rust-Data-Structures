#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

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
}

impl<T: Eq + Hash> SmartSet<T> {
    pub fn add(&mut self, item: T)
    {
        self.items.insert(item);
    }

    pub fn get(&self, item: &T) -> Option<&T>
    {
        self.items.get(item)
    }

    pub fn contains(&self, item: &T) -> bool
    {
        self.items.contains(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut set: SmartSet<i32> = SmartSet::new();

        set.add(1);
        set.add(2);
        set.add(3);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_clear() {
        let mut set: SmartSet<i32> = SmartSet::new();
        assert!(set.is_empty());

        set.add(1);
        set.add(2);
        set.add(3);

        assert!(!set.is_empty());
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.add(1);
        set.add(2);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_get() {
        let mut set: SmartSet<i32> = SmartSet::new();
        set.add(1);
        set.add(2);

        assert_eq!(set.get(&1), Some(&1));
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&3), None);
    }

    #[test]
    fn test_is_empty() {
        let set: SmartSet<i32> = SmartSet::new();
        assert!(set.is_empty());
    }
}