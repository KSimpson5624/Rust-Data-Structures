# Rust Data Structures

This repo houses all of my custom Rust data structures. 

## Counter
This data structure is a hashmap that accepts keys only as input and increments the value based on the number of times the key is inserted. 
For example:
`
let mut counter: Counter<&str> = Counter::new();
counter.add("A");
counter.add("A");
counter.add("B");

assert_eq!(counter.get("A"), Some(&2));
`

# Future plans
I plan to design a SmartSet next, it will have almost the same performance as a HashSet but can be ordered on demand. 
