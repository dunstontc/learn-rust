# [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html)

- Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.
- Just like vectors, hash maps store their data on the heap. 


## Creating a New Hash Map
- `HashMap<K, V>`
- You can create an empty hash map with new and add elements with insert. 
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);  
```


## Hash Maps and Ownership
- For types that implement the Copy trait, like i32, the values are copied into the hash map. 
- For owned values like String, the values will be moved and the hash map will be the owner of those values.
- If we insert references to values into the hash map, the values won’t be moved into the hash map. 
- The values that the references point to must be valid for at least as long as the hash map is valid. 


## Accessing Values in a Hash Map
- We can get a value out of the hash map by providing its key to the get method.
- We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors.
```rust
// This code will print each pair in an arbitrary order
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```


## Updating a Hash Map
- Although the number of keys and values is growable, each key can only have one value associated with it at a time. When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. Or you could combine the old value and the new value. 

### Overwriting a Value
- If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

### Only Inserting a Value If the Key Has No Value
- It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it. 
Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
The return value of the entry function is an enum called Entry that represents a value that might or might not exist.
```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```
The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

### Updating a Value Based on the Old Value


## Hashing Functions
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

