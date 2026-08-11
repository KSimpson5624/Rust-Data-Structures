# Rust Data Structures

This repo houses all of my custom Rust data structures. 

## Counter
This data structure is a hashmap that accepts keys only as input and increments the value based on the number of times the key is inserted. 
For example:
```rust
let mut counter: Counter<&str> = Counter::new();
counter.add("A");
counter.add("A");
counter.add("B");

assert_eq!(counter.get("A"), Some(&2));
```
## SmartSet
This data structure is a hybrid model that maintains the performance and behavior of a HashSet, while being able to be ordered on demand. 

Example:
```rust
let mut set: SmartSet<i32> = SmartSet::new();
set.insert(5);
set.insert(1);
set.insert(3);
set.sort();

for item in &set {
    println!("{}", item); // prints out 1, 3, 5 in order
}
assert!(set.contains(3)); // Still maintains O(1) for lookups
```
