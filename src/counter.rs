
use std::borrow::Borrow;
use std::collections::{hash_map, HashMap};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{AddAssign, SubAssign};
use std::fmt;

/// # Counter
///
/// Underlying structure is a hashmap, it increments the value by 1 for every occurrence of the key.
#[derive(Debug, PartialEq)]
pub struct Counter<T>
where
    T: Eq + Hash,
{
    counts: HashMap<T, usize>,
}

impl<T: Eq + Hash> Default for Counter<T> {
    fn default() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
            Self::default()
        }

    /// Checks the length of a Counter
    ///
    /// Returns the number of keys in the Counter<br>
    /// Return type is usize
    ///
    /// # Example
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<&str> = Counter::new();
    /// counter.add("A");
    /// counter.add("B");
    /// counter.add("C");
    /// counter.add("A");
    ///
    /// assert_eq!(counter.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Checks if a Counter has any keys
    ///
    /// Checks if the Counter is empty
    /// Returns a boolean<br>
    ///
    /// # Example
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<&str> = Counter::new();
    ///
    /// assert!(counter.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Adds a key or increments count of present key
    ///
    /// If a key is already present, then add increments the count by 1.<br>
    /// If a key is not present, then add inserts the key and sets the count to 1.
    ///
    /// # Example
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<&str> = Counter::new();
    ///
    /// counter.add("A");
    /// assert_eq!(counter.get("A"), Some(&1));
    ///
    /// counter.add("A");
    /// assert_eq!(counter.get("A"), Some(&2));
    /// ```
    pub fn add(&mut self, item: T) {
        self.counts.entry(item).and_modify(|count| *count += 1).or_insert(1);
    }

    /// Subtracts the count of a key
    ///
    /// Decrease the count of a key by 1. If the count reaches zero, then
    /// the key is removed entirely.
    ///
    /// Nothing occurs if the key is not present.
    ///
    /// # Examples:
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<char> = "rustisgreat".chars().collect();
    ///
    /// assert_eq!(counter.get(&'t'), Some(&2));
    /// counter.subtract('t');
    /// assert_eq!(counter.get(&'t'), Some(&1));
    /// assert_eq!(counter.get(&'d'), None);
    ///```
    pub fn subtract(&mut self, item: T) {
        match self.counts.entry(item) {
            Entry::Occupied(mut entry) => {
                let count = entry.get_mut();
                *count = count.saturating_sub(1);

                if *count == 0 {
                    entry.remove();
                }
            }
            Entry::Vacant(_) => {}
        }
    }

    /// Gets the count of a key
    ///
    /// Returns the count of the key as an Option<&usize>
    ///
    /// # Example:
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<char> = "rust".chars().collect();
    /// let r_count = match counter.get(&'r') {
    ///     Some(&r_count) => r_count,
    ///     None => 0,
    /// };
    ///
    /// assert_eq!(r_count, 1)
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&usize>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.counts.get(key)
    }

    /// Subtracts the counts of another `Counter` from this one.
    ///
    /// For each key in `other`, its count is subtracted from the corresponding count in `self`.
    /// Keys whose count reaches zero are removed entirely. Keys present in `self` but not in
    /// `other` are left unchanged.
    ///
    /// # Example:
    /// ```
    /// use data_structures::Counter;
    /// let mut counter1: Counter<char> = "rust".chars().collect();
    /// let mut counter2: Counter<char> = "rustacean".chars().collect();
    ///
    /// counter2.subtract_counter(&counter1);
    ///
    /// assert_eq!(counter2.get(&'r'), None);
    /// assert_eq!(counter2.get(&'u'), None);
    /// assert_eq!(counter2.get(&'s'), None);
    /// assert_eq!(counter2.get(&'t'), None);
    /// assert_eq!(counter2.get(&'a'), Some(&2));
    /// assert_eq!(counter2.get(&'c'), Some(&1));
    /// assert_eq!(counter2.get(&'e'), Some(&1));
    /// assert_eq!(counter2.get(&'n'), Some(&1));
    /// ```
    pub fn subtract_counter(&mut self, other: &Counter<T>)
    where
        T: Clone,
    {
        for (key, &value) in &other.counts {
            match self.counts.entry(key.clone()) {
                Entry::Occupied(mut entry) => {
                    let count = entry.get_mut();
                    *count = count.saturating_sub(value);

                    if *count == 0 {
                        entry.remove();
                    }
                }
                Entry::Vacant(_) => {}
            }
        }
    }

    /// Adds the counts of another `Counter` to this one
    ///
    /// For each key in `other`, its count is added to teh corresponding count in `self`.
    /// Keys present in `other` but not in `self` are inserted with `other`'s count.
    ///
    /// Counts saturate at [`usize::MAX`] rather than overflowing.
    ///
    /// # Example:
    /// ```
    /// use data_structures::Counter;
    /// let mut counter1: Counter<char> = "rust".chars().collect();
    /// let mut counter2: Counter<char> = "rustacean".chars().collect();
    ///
    /// counter1.add_counter(&counter2);
    ///
    /// assert_eq!(counter1.get(&'r'), Some(&2));
    /// assert_eq!(counter1.get(&'u'), Some(&2));
    /// assert_eq!(counter1.get(&'s'), Some(&2));
    /// assert_eq!(counter1.get(&'t'), Some(&2));
    /// assert_eq!(counter1.get(&'a'), Some(&2));
    /// assert_eq!(counter1.get(&'c'), Some(&1));
    /// assert_eq!(counter1.get(&'e'), Some(&1));
    /// assert_eq!(counter1.get(&'n'), Some(&1));
    /// ```
    ///
    pub fn add_counter(&mut self, other: &Counter<T>)
    where
        T: Clone,
    {
        for (key, &value) in &other.counts {
            let entry = self.counts.entry(key.clone()).or_insert(0);
            *entry = entry.saturating_add(value)
        }
    }

    /// Removes key from Counter entirely
    ///
    /// Removes key and its count (regardless of count value) from `Counter`.
    /// If key does not exist, then nothing happens.
    ///
    /// # Example
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<char> = Counter::new();
    /// counter.add('a');
    /// counter.add('a');
    /// counter.add('b');
    /// counter.add('c');
    ///
    /// counter.remove(&'a');
    /// assert_eq!(counter.get(&'a'), None);
    /// assert_eq!(counter.get(&'b'), Some(&1));
    /// assert_eq!(counter.get(&'c'), Some(&1));
    /// ```
    pub fn remove<Q>(&mut self, key: &Q)
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.counts.remove(key);
    }

    /// Finds the key with the highest count
    ///
    /// Returns the name of the key with the highest count.
    /// If multiple keys have the same count, it takes the first one encountered.
    ///
    /// Please note: "first encountered" can be different every time since a `Counter` is
    /// unordered.
    ///
    /// Returns `None` if `Counter` is empty
    ///
    /// # Example:
    /// ```
    /// use data_structures::Counter;
    /// let counter1: Counter<char> = "rustacean".chars().collect();
    /// let counter2: Counter<char> = Counter::new();
    ///
    /// assert_eq!(counter1.most_common(), Some(&'a'));
    /// assert_eq!(counter2.most_common(), None);
    /// ```
    pub fn most_common(&self) -> Option<&T> {
        self.counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(key, _)| key)
    }

    /// Finds the key with the lowest count
    ///
    /// Returns the name of the key with the lowest count.
    /// If multiple keys have the same count, it takes the first one encountered.
    ///
    /// Please note: "first encountered" can be different every time since a `Counter` is
    /// unordered.
    ///
    /// Returns `None` if `Counter` is empty
    ///
    /// # Example:
    /// ```
    /// use data_structures::Counter;
    /// let mut counter: Counter<char> = "abbccc".chars().collect();
    ///
    /// assert_eq!(counter.least_common(), Some(&'a'));
    /// ```
    pub fn least_common(&self) -> Option<&T> {
        self.counts
            .iter()
            .min_by_key(|(_, count)| *count)
            .map(|(key, _)| key)
    }
    /*
    pub fn highest_count(&self) -> Option<&T> {

    }

     */
}

/// Subtracts the counts of another `Counter` from this one in place.
///
/// Keys whose count reaches zero are removed. See [`Counter::subtract_counter`] for more details.
impl<T: Eq + Hash + Clone> SubAssign<&Counter<T>> for Counter<T> {
    fn sub_assign(&mut self, rhs: &Counter<T>) {
        self.subtract_counter(rhs);
    }
}

/// Adds the counts of another `Counter` to this one in place.
///
/// Counts saturate at [`usize::MAX`]. See [`Counter::add_counter`] for more details.
impl<T: Eq + Hash + Clone> AddAssign<&Counter<T>> for Counter<T> {
    fn add_assign(&mut self, rhs: &Counter<T>) {
        self.add_counter(rhs);
    }
}

/// Creates a `Counter` from an iterator by counting the occurrences of each element.
///
/// # Example
/// ```
/// use data_structures::Counter;
///
/// let counter: Counter<char> = "rust".chars().collect();
///
/// assert_eq!(counter.get(&'r'), Some(&1));
/// assert_eq!(counter.get(&'u'), Some(&1));
/// assert_eq!(counter.get(&'s'), Some(&1));
/// assert_eq!(counter.get(&'t'), Some(&1));
/// ```
impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter = Counter::new();

        for item in iter {
            counter.add(item);
        }

        counter
    }
}

impl<T> IntoIterator for Counter<T>
where
    T: Eq + Hash,
{
    type Item = (T, usize);
    type IntoIter = hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Counter<T>
where
    T: Eq + Hash,
{
    type Item = (&'a T, &'a usize);
    type IntoIter = hash_map::Iter<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Counter<T>
where
    T: Eq + Hash,
{
    type Item = (&'a T, &'a mut usize);
    type IntoIter = hash_map::IterMut<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.iter_mut()
    }
}

impl<T> fmt::Display for Counter<T>
where
    T: fmt::Display + Eq + Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        let mut first = true;

        for (key, value) in &self.counts {
            if !first {
                write!(f, ", ")?;
            }
            first = false;

            write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("A");
        counter.add("B");
        counter.add("C");

        assert_eq!(counter.get(&"A"), Some(&1));
        assert_eq!(counter.get(&"B"), Some(&1));
        assert_eq!(counter.get(&"C"), Some(&1));
        assert_eq!(counter.get(&"D"), None);
    }

    #[test]
    fn test_subtract() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("A");
        counter.add("B");
        counter.add("C");

        assert_eq!(counter.get("A"), Some(&1));
        assert_eq!(counter.get("B"), Some(&1));
        assert_eq!(counter.get("C"), Some(&1));

        counter.subtract("C");
        assert_eq!(counter.get("C"), None);
        assert_eq!(counter.get("B"), Some(&1));
        assert_eq!(counter.get("A"), Some(&1));

        counter.subtract("K");
        assert_eq!(counter.get("K"), None);
    }

    #[test]
    fn test_len() {
        let mut counter: Counter<char> = Counter::new();

        for character in "areallylongword".chars() {
            counter.add(character);
        }

        assert_eq!(counter.len(), 10);
    }

    #[test]
    fn test_is_empty() {
        let counter: Counter<&str> = Counter::new();

        assert!(counter.is_empty());
    }

    #[test]
    fn test_add_counter() {
        let mut counter1 = Counter::new();
        let mut counter2 = Counter::new();
        counter1.add("e");
        counter1.add("e");
        counter1.add("k");

        counter2.add("e");
        counter2.add("k");
        counter2.add("f");

        counter1.add_counter(&counter2);

        assert_eq!(counter1.get("e"), Some(&3));
        assert_eq!(counter1.get("k"), Some(&2));
        assert_eq!(counter1.get("f"), Some(&1));
    }

    #[test]
    fn test_subtract_counter() {
        let mut counter1: Counter<&str> = Counter::new();
        let mut counter2: Counter<&str> = Counter::new();
        counter1.add("e");
        counter1.add("e");
        counter1.add("k");
        counter1.add("f");

        counter2.add("k");
        counter2.add("f");

        counter1.subtract_counter(&counter2);

        assert_eq!(counter1.get("k"), None);
        assert_eq!(counter1.get("f"), None);
        assert_eq!(counter1.get("e"), Some(&2));
    }

    #[test]
    fn test_add_operator() {
        let mut counter1: Counter<&str> = Counter::new();
        let mut counter2: Counter<&str> = Counter::new();

        counter1.add("e");
        counter1.add("k");
        counter1.add("e");
        counter1.add("d");

        counter2.add("b");
        counter2.add("k");

        counter1 += &counter2;

        assert_eq!(counter1.get("e"), Some(&2));
        assert_eq!(counter1.get("k"), Some(&2));
        assert_eq!(counter1.get("d"), Some(&1));
        assert_eq!(counter1.get("b"), Some(&1));
    }

    #[test]
    fn test_subtract_operator() {
        let mut counter1: Counter<&str> = Counter::new();
        let mut counter2: Counter<&str> = Counter::new();

        counter1.add("e");
        counter1.add("k");
        counter1.add("e");
        counter1.add("d");

        counter2.add("b");
        counter2.add("k");

        counter1 -= &counter2;

        assert_eq!(counter1.get("e"), Some(&2));
        assert_eq!(counter1.get("k"), None);
        assert_eq!(counter1.get("d"), Some(&1));
        assert_eq!(counter1.get("b"), None);
    }

    #[test]
    fn test_no_negatives() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("e");
        counter.add("k");
        counter.add("f");

        counter.subtract("e");
        counter.subtract("e");

        assert_eq!(counter.get("e"), None);
    }

    #[test]
    fn test_using_integers() {
        let mut counter: Counter<i32> = Counter::new();

        counter.add(1);
        counter.add(2);
        counter.add(3);
        counter.add(1);

        assert_eq!(counter.get(&1), Some(&2));
        assert_eq!(counter.get(&2), Some(&1));
        assert_eq!(counter.get(&3), Some(&1))

    }

    #[test]
    fn test_using_chars() {
        let mut counter: Counter<char> = Counter::new();

        for character in "aabccc".chars() {
            counter.add(character);
        }

        assert_eq!(counter.get(&'a'), Some(&2));
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'c'), Some(&3));
    }

    #[test]
    fn test_using_strings() {
        let mut counter: Counter<String> = Counter::new();

        counter.add(String::from("Football"));
        counter.add(String::from("Quarterback"));
        counter.add(String::from("Touchdown"));
        counter.add(String::from("Football"));
        counter.add(String::from("Quarterbacks"));

        assert_eq!(counter.get("Football"), Some(&2));
        assert_eq!(counter.get("Quarterback"), Some(&1));
        assert_eq!(counter.get("Touchdown"), Some(&1));
        assert_eq!(counter.get("Quarterbacks"), Some(&1));
    }

    #[test]
    fn test_add_1000_times() {
        let mut counter: Counter<&str> = Counter::new();

        for _ in 0..1000 {
            counter.add("A");
        }
        assert_eq!(counter.get("A"), Some(&1000));
        assert_eq!(counter.len(), 1);
        assert_eq!(counter.get("B"), None);
    }

    #[test]
    fn test_from_iterator() {
        let counter: Counter<char> = "banana".chars().collect();

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'a'), Some(&3));
        assert_eq!(counter.get(&'n'), Some(&2));
    }

    #[test]
    fn test_from_iterator_with_vec() {
        let counter: Counter<i32> = Counter::from_iter(vec![1, 2, 3, 1, 1, 3]);

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.get(&1), Some(&3));
        assert_eq!(counter.get(&2), Some(&1));
        assert_eq!(counter.get(&3), Some(&2));
    }

    #[test]
    fn test_into_iterator_consuming() {
        let counter: Counter<char> = "banana".chars().collect();

        let mut map: HashMap<char, usize> = HashMap::new();

        for (key, value) in counter {
            map.insert(key, value);
        }

        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('b', 1);
        expected.insert('a', 3);
        expected.insert('n', 2);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_into_iterator_borrowed() {
        let counter: Counter<char> = "banana".chars().collect();

        let mut map: HashMap<char, usize> = HashMap::new();

        for (key, value) in &counter {
            map.insert(*key, *value);
        }

        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('b', 1);
        expected.insert('a', 3);
        expected.insert('n', 2);

        assert_eq!(map, expected);
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.len(), map.len());
    }

    #[test]
    fn test_into_iterator_borrowed_mut() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("banana");
        counter.add("strawberry");
        counter.add("apple");

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.get("banana"), Some(&1));
        assert_eq!(counter.get("strawberry"), Some(&1));
        assert_eq!(counter.get("apple"), Some(&1));

        for (_, value) in &mut counter {
            *value += 10;
        }

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.get("banana"), Some(&11));
        assert_eq!(counter.get("strawberry"), Some(&11));
        assert_eq!(counter.get("apple"), Some(&11));
    }

    #[test]
    fn test_remove() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("A");
        counter.add("B");
        counter.add("C");
        counter.add("A");

        counter.remove("A");

        assert_eq!(counter.len(), 2);
        assert_eq!(counter.get("A"), None);
        assert_eq!(counter.get("B"), Some(&1));
        assert_eq!(counter.get("C"), Some(&1));
    }

    #[test]
    fn test_remove_all() {
        let mut counter: Counter<&str> = Counter::new();

        counter.add("A");
        counter.add("B");
        counter.add("C");
        counter.add("A");

        counter.remove("A");
        counter.remove("B");
        counter.remove("C");

        assert!(counter.is_empty());
    }

    #[test]
    fn test_most_common() {
        let counter: Counter<char> = "banana".chars().collect();

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.most_common(), Some(&'a'));
    }

    #[test]
    fn test_most_common_on_empty() {
        let counter: Counter<char> = Counter::new();

        assert!(counter.is_empty());
        assert_eq!(counter.most_common(), None);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut counter: Counter<char> = "rust".chars().collect();

        counter.remove(&'B');

        assert_eq!(counter.len(), 4);
        assert_eq!(counter.get(&'B'), None);
        assert_eq!(counter.get(&'r'), Some(&1));
        assert_eq!(counter.get(&'u'), Some(&1));
        assert_eq!(counter.get(&'s'), Some(&1));
        assert_eq!(counter.get(&'t'), Some(&1));
    }
    #[test]
    fn test_least_common() {
        let mut counter: Counter<&str> = Counter::new();
        counter.add("banana");
        counter.add("apple");
        counter.add("strawberry");
        counter.add("apple");
        counter.add("banana");

        assert_eq!(counter.len(), 3);
        assert_eq!(counter.least_common(), Some(&"strawberry"));
    }

    #[test]
    fn test_least_common_on_tie() {
        let counter: Counter<char> = "rust".chars().collect();
        let valid = vec!['r', 'u', 's', 't'];
        assert!(valid.contains(&counter.least_common().unwrap()));
    }

    #[test]
    fn test_least_common_on_empty() {
        let counter: Counter<&str> = Counter::new();

        assert!(counter.is_empty());
        assert_eq!(counter.least_common(), None);
    }
}