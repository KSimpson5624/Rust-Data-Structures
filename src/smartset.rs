#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::hash_set::{Drain, Iter};
use std::collections::{hash_set, HashSet};
use std::hash::Hash;

/// SmartSet
///
/// Underlying structure is a HashSet. It keeps the O(1) performance of a HashSet, while allowing for
/// ordering on demand.
#[derive(Debug)]
pub struct SmartSet<T> {
    items: HashSet<T>,
    cache: Option<Vec<T>>,
    is_sorted: bool,
}

impl<T> SmartSet<T> {
    /// Creates an empty SmartSet
    ///
    /// Initial capacity of an empty SmartSet is 0.
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            cache: None,
            is_sorted: false,
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
            is_sorted: false,
        }
    }

    /// Generates an iterator over the `SmartSet`
    ///
    /// # Example
    /// ```
    /// use data_structures::SmartSet;
    /// let set: SmartSet<char> = "rust".chars().collect();
    ///
    /// let mut iterator = set.iter();
    /// iterator.next(); // Will be 'r', 'u', 's', or 't' in a random order
    /// iterator.next(); // Will be 'r', 'u', 's', or 't' in a random order
    /// iterator.next(); // Will be 'r', 'u', 's', or 't' in a random order
    /// iterator.next(); // Will be 'r', 'u', 's', or 't' in a random order
    /// iterator.next(); // Will be `None`
    /// ```
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
    /// Example:
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

/// Creates a `SmartSet` from an iterator by counting the occurrences of each element.
///
/// Duplicates will be removed
///
/// # Example
/// ```
/// use data_structures::SmartSet;
///
/// let set: SmartSet<char> = "rust".chars().collect();
///
/// assert_eq!(set.get(&'r'), Some(&'r'));
/// assert_eq!(set.get(&'u'), Some(&'u'));
/// assert_eq!(set.get(&'s'), Some(&'s'));
/// assert_eq!(set.get(&'t'), Some(&'t'));
/// ```
impl<T> FromIterator<T> for SmartSet<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut set = Self::new();

        for item in iter {
            set.insert(item);
        }
        set
    }
}

/// Creates a consuming iterator for `SmartSet`
///
/// # Example
/// ```
/// use data_structures::SmartSet;
///
/// let set: SmartSet<char> = "rust".chars().collect();
///
/// for character in set {
///     println!("{}", character); // Will print out 'r', 'u', 's', 't' in a random order.
/// }
/// ```
impl<T> IntoIterator for SmartSet<T>
where
    T: Eq + Hash,
{
    type Item = T;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a SmartSet<T>
where
    T: Eq + Hash,
{
    type Item = &'a T;
    type IntoIter = hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
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

    #[test]
    fn test_from_iter() {
        let set = SmartSet::from_iter(vec![1, 2, 3]);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_collect() {
        let set: SmartSet<char> = "Hello".chars().collect();
        assert_eq!(set.len(), 4);
        assert!(set.contains(&'H'));
        assert!(set.contains(&'e'));
        assert!(set.contains(&'l'));
        assert!(set.contains(&'o'));
    }

    #[test]
    fn test_into_iter() {
        let set: SmartSet<char> = "rust".chars().collect();

        let mut iter = set.into_iter();
        assert!(vec!['r', 'u', 's', 't'].contains(&iter.next().unwrap()));
        assert!(vec!['r', 'u', 's', 't'].contains(&iter.next().unwrap()));
        assert!(vec!['r', 'u', 's', 't'].contains(&iter.next().unwrap()));
        assert!(vec!['r', 'u', 's', 't'].contains(&iter.next().unwrap()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_for_loop() {
        let set: SmartSet<char> = "rust".chars().collect();
        for character in set {
            assert!(vec!['r', 'u', 's', 't'].contains(&character));
        }
    }

    #[test]
    fn test_into_iter_with_reference() {
        let set: SmartSet<char> = "rust".chars().collect();

        for character in &set {
            assert!(vec!['r', 'u', 's', 't'].contains(&character));
        }
    }
}