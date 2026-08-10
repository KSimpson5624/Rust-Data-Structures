
use std::borrow::Borrow;
use std::collections::hash_set::Drain;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

pub enum SmartSetIntoIter<T> {
    Set(std::collections::hash_set::IntoIter<T>),
    Sorted(std::vec::IntoIter<T>),
}

pub enum SmartSetIter<'a, T> {
    Set(std::collections::hash_set::Iter<'a, T>),
    Sorted(std::slice::Iter<'a, T>),
}

/// SmartSet
///
/// Underlying structure is a HashSet. It keeps the O(1) performance of a HashSet, while allowing for
/// ordering on demand.
#[derive(Debug, Clone)]
pub struct SmartSet<T> {
    items: HashSet<T>,
    cache: Option<Vec<T>>,
    is_sorted: bool,
    item_removed: bool,
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
            item_removed: false,
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
        self.is_sorted = false;
        self.item_removed = false;
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

    pub fn is_sorted(&self) -> bool {
        self.is_sorted
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
            item_removed: false,
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
    pub fn iter(&self) -> SmartSetIter<'_, T> {
        if !self.is_sorted {
            SmartSetIter::Set(self.items.iter())
        } else {
            SmartSetIter::Sorted(self.cache.as_ref().unwrap().iter())
        }
    }

    /// `drain` returns all the elements from a `SmartSet` and removes them from the `SmartSet`
    ///
    /// # Note: Even if the `SmartSet` is sorted, `drain` will return the elements in an arbitrary order.
    ///
    /// # Example:
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::from_iter(vec![1, 2, 3, 4, 5]);
    /// let other_set = set.drain();
    ///
    /// // assert!(set.is_empty()); This would return true if it didn't violate borrowing rules
    ///
    /// for item in other_set {
    ///     println!("{}", item); // 1, 2, 3, 4, 5 in an arbitrary order
    /// }
    /// ```
    pub fn drain(&mut self) -> Drain<'_, T> {
        self.cache = None;
        self.is_sorted = false;
        self.item_removed = false;
        self.items.drain()
    }

}

impl<T> Default for SmartSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash> SmartSet<T> {
    /// Inserts an item into the `SmartSet`
    ///
    /// This will cause the `SmartSet` to revert back to unsorted
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
    where
        T: Clone,
    {
        if self.cache.is_some() && !self.item_removed {
            // This unwrap is safe because the condition above confirms self.cache is not None
            self.cache.as_mut().unwrap().push(item.clone());
        }
        self.is_sorted = false;
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

    /// Removes an item from `SmartSet`
    ///
    /// This will cause `SmartSet` to revert back to unsorted
    ///
    /// # Example
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<i32> = SmartSet::new();
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    ///
    /// assert!(set.contains(&2));
    /// set.remove(&2);
    /// assert!(!set.contains(&2));
    /// ```
    pub fn remove<Q>(&mut self, item: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.cache.is_some() {
            self.item_removed = true;
        }
        self.is_sorted = false;
        self.items.remove(item)
    }

    /// Removes and returns and item from a `SmartSet`
    ///
    /// This will cause a `SmartSet` to revert back to unsorted
    ///
    /// This returns `Option<T>`
    ///
    /// # Example
    /// ```
    /// use data_structures::SmartSet;
    /// let mut set: SmartSet<char> = "rust".chars().collect();
    /// let r = set.take(&'r').unwrap();
    /// println!("{}", r);
    /// ```
    pub fn take<Q>(&mut self, item: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.cache.is_some() {
            self.item_removed = true;
        }
        self.is_sorted = false;
        self.items.take(item)
    }

    pub fn sort(&mut self)
    where
        T: Ord + Clone,
    {
        // If the cache doesn't exist yet or if an item has been removed we want to clone every item form the underlying hashset prior
        // to sorting. Removal of an item in a Vector is O(n), which is the same as re-cloning the entire set. This will reduce the cost
        // of maintaining the cache
        if self.cache.is_none() || self.item_removed {
            self.cache = Some(self.items.iter().cloned().collect());
        }
        // This unwrap is safe because if the cache is None, it gets assigned in the conditional above
        self.cache.as_mut().unwrap().sort();
        self.is_sorted = true;
    }

    pub fn sort_unstable(&mut self)
    where
    T: Ord + Clone
    {
        // If the cache doesn't exist yet or if an item has been removed we want to clone every item form the underlying hashset prior
        // to sorting. Removal of an item in a Vector is O(n), which is the same as re-cloning the entire set. This will reduce the cost
        // of maintaining the cache
        if self.cache.is_none() || self.item_removed {
            self.cache = Some(self.items.iter().cloned().collect());
        }
        // This unwrap is safe because if the cache is None, it gets assigned in the conditional above
        self.cache.as_mut().unwrap().sort_unstable();
        self.is_sorted = true;
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
    T: Eq + Hash + Clone,
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self
    {
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
    type IntoIter = SmartSetIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        if !self.is_sorted {
            SmartSetIntoIter::Set(self.items.into_iter())
        } else {
            SmartSetIntoIter::Sorted(self.cache.unwrap().into_iter())
        }
    }
}

impl<'a, T> Iterator for SmartSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SmartSetIter::Set(iter) => iter.next(),
            SmartSetIter::Sorted(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            SmartSetIter::Set(iter) => iter.size_hint(),
            SmartSetIter::Sorted(iter) => iter.size_hint(),
        }
    }
}
impl<T> Iterator for SmartSetIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SmartSetIntoIter::Set(iter) => iter.next(),
            SmartSetIntoIter::Sorted(iter) => iter.next(),
        }
    }
}

impl<'a, T> IntoIterator for &'a SmartSet<T>
where
    T: Eq + Hash,
{
    type Item = &'a T;
    type IntoIter = SmartSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> fmt::Display for SmartSet<T>
where
    T: fmt::Display + Eq + Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}", item)?;
        }
        write!(f, "}}")
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
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
        let set: SmartSet<i32> = SmartSet::with_capacity(10);
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
            assert!(vec![1, 2, 3].contains(&i));
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

    #[test]
    fn test_unstable_sort() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort_unstable();
        let mut iterator = set.into_iter();
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), Some(8));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_size_hint_empty_set() {
        let set: SmartSet<i32> = SmartSet::new();
        let iterator = set.iter();
        assert_eq!((0, Some(0)), iterator.size_hint())
    }

    #[test]
    fn test_size_hint() {
        let set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let iterator = set.iter();
        assert_eq!((6, Some(6)), iterator.size_hint());
    }

    #[test]
    fn test_size_hint_after_consuming() {
        let set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let mut iterator = set.iter();
        iterator.next();
        iterator.next();
        assert_eq!((4, Some(4)), iterator.size_hint());
    }

    #[test]
    fn test_sort() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let expected = vec![1, 2, 3, 4, 5, 8];
        set.sort();
        for (index, value) in set.iter().enumerate() {
            assert_eq!(*value, expected[index]);
        }
    }

    #[test]
    fn test_sort_after_insert() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let not_expected = vec![1, 2, 3, 4, 5, 8, 9];
        set.sort();
        set.insert(9);
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_sort_after_remove() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let not_expected = vec![1, 2, 3, 4, 8];
        set.sort();
        set.remove(&5);
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_sort_unstable_after_insert() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let not_expected = vec![1, 2, 3, 4, 5, 8, 9];
        set.sort_unstable();
        set.insert(9);
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_sort_unstable_after_remove() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let not_expected = vec![1, 2, 3, 4, 8];
        set.sort_unstable();
        set.remove(&5);
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_remove() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.remove(&5);
        set.remove(&2);
        set.remove(&4);
        assert_eq!(set.len(), 3);
        assert!(!set.contains(&5));
        assert!(set.contains(&8));
        assert!(!set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&1));
        assert!(!set.contains(&4));
    }

    #[test]
    fn test_take() {
        let mut set: SmartSet<char> = "rust".chars().collect();
        let r = set.take(&'r');
        assert_eq!(r, Some('r'));
    }

    #[test]
    fn test_sort_after_take() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        let one = set.take(&1);
        assert_eq!(one, Some(1));
        let not_expected = vec![2, 3, 4, 5, 8];
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_sort_unstable_after_take() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort_unstable();
        let one = set.take(&1);
        assert_eq!(one, Some(1));
        let not_expected = vec![2, 3, 4, 5, 8];
        assert_ne!(set.iter().cloned().collect::<Vec<i32>>(), not_expected);
    }

    #[test]
    fn test_sort_twice() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let expected = vec![1, 2, 3, 4, 5, 8];
        set.sort();
        for (index, value) in set.iter().enumerate() {
            assert_eq!(*value, expected[index]);
        }
        set.sort();
        for (index, value) in set.iter().enumerate() {
            assert_eq!(*value, expected[index]);
        }
    }

    #[test]
    fn test_sort_on_pathbuf() {
        let paths: [PathBuf;4] = [
            PathBuf::from("C:/Users/root"),
            PathBuf::from("C:/Users/root/Desktop"),
            PathBuf::from("C:/Users/root/Documents"),
            PathBuf::from("C:/Users/root/Downloads"),
        ];

        let mut set: SmartSet<PathBuf> = SmartSet::from_iter(paths.clone());
        set.sort();
        assert_eq!(set.iter().cloned().collect::<Vec<PathBuf>>(), paths);
    }

    #[test]
    fn test_is_sorted() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
    }

    #[test]
    fn test_is_sorted_after_insert() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
        set.insert(9);
        assert!(!set.is_sorted());
    }

    #[test]
    fn test_is_sorted_after_remove() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
        set.remove(&4);
        assert!(!set.is_sorted());
    }

    #[test]
    fn test_is_sorted_after_take() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
        let _four = set.take(&4);
        assert!(!set.is_sorted());
    }

    #[test]
    fn test_is_sorted_after_get() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
        let _four = set.get(&4);
        assert!(set.is_sorted());
    }

    #[test]
    fn test_is_sorted_after_iteration() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        set.sort();
        assert!(set.is_sorted());
        for item in &set {
            println!("{:?}", item);
        }
        assert!(set.is_sorted());
    }

    #[test]
    fn test_sorted_length_after_insertion() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let pre_sort_length = set.len();
        set.sort();
        set.insert(9);
        set.sort();
        assert_eq!(set.len(), (pre_sort_length + 1));
    }

    #[test]
    fn test_sorted_length_after_removal() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let pre_sort_length = set.len();
        set.sort();
        set.remove(&4);
        set.sort();
        assert_eq!(set.len(), (pre_sort_length - 1));
    }

    #[test]
    fn test_unstable_sorted_length_after_insertion() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let pre_sort_length = set.len();
        set.sort_unstable();
        set.insert(9);
        set.sort_unstable();
        assert_eq!(set.len(), (pre_sort_length + 1));
    }

    #[test]
    fn test_unstable_sorted_length_after_removal() {
        let mut set: SmartSet<i32> = SmartSet::from_iter(vec![5, 8, 4, 2, 1, 3]);
        let pre_sort_length = set.len();
        set.sort_unstable();
        set.remove(&4);
        set.sort_unstable();
        assert_eq!(set.len(), (pre_sort_length - 1));
    }
}