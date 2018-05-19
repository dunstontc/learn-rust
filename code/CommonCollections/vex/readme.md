# [Common Collections](https://doc.rust-lang.org/book/second-edition/ch08-00-common-collections.html)


- A *vector* allows you to store a variable number of values next to each other in memory.
- A *string* is a collection of characters. 
- A *hash map* allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a *map*.
- [std::collections](https://doc.rust-lang.org/std/collections/index.html)


## Vectors
- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 
- Vectors are implemented using generics.
- The `Vec<T>` type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets.
- Like any other struct, a vector is freed when it goes out of scope.
- As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

```rust
// To create a new, empty vector, we can call the `Vec::new()` function.
let v: Vec<i32> = Vec::new();

// It’s more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro for convenience. 
{
    let v = vec![1, 2, 3, 4];
} // <- v goes out of scope and is freed here

let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Creating a New Vector
- `Vec::new()`
- `vec![...]`

### Updating a Vector
- `v.push(x)`

### Reading Elements of Vectors
- `&v[n]`
- `v.get(n)`

### Iterating over the Values in a Vector
```rust
let u = vec![100, 32, 57];
for i in &u {
    println!("{}", i);
}

let mut m = vec![100, 32, 57];
for i in &mut m {
    *i += 50;
}
```

### Using an Enum to Store Multiple Types
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

