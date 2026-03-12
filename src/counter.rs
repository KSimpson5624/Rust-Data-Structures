
use std::borrow::Borrow;
use std::collections::{hash_map, HashMap};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{AddAssign, SubAssign};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Counter<T>
where
    T: Eq + Hash,
{
    counts: HashMap<T, usize>,
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    pub fn add(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }

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

    pub fn get<Q>(&self, key: &Q) -> Option<&usize>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.counts.get(key)
    }

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

    pub fn add_counter(&mut self, other: &Counter<T>)
    where
        T: Clone,
    {
        for (key, &value) in &other.counts {
            let entry = self.counts.entry(key.clone()).or_insert(0);
            *entry = entry.saturating_add(value)
        }
    }
}

impl<T: Eq + Hash + Clone> SubAssign<&Counter<T>> for Counter<T> {
    fn sub_assign(&mut self, rhs: &Counter<T>) {
        self.subtract_counter(rhs);
    }
}

impl<T: Eq + Hash + Clone> AddAssign<&Counter<T>> for Counter<T> {
    fn add_assign(&mut self, rhs: &Counter<T>) {
        self.add_counter(rhs);
    }
}

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
}