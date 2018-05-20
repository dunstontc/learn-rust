# [Ownership in Rust](https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html)

## Ownership Rules
- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Concepts
- Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
- When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
- Types like integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

## Copy
If a type has the `Copy` trait, an older variable is still usable after assignment.  
Rust won’t let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the `Drop` trait.  
As a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`.  
Here are some of the types that are `Copy`:  

- All the integer types, like `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- The character type, `char`.
- All the floating point types, like `f64`.
- Tuples, but only if they contain types that are also `Copy`. (`i32`, `i32`) is `Copy`, but (`i32`, `String`) is not.

## Ownership & Functions
- Passing a variable to a function will move or copy, just like assignment.
- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

## References
1. At any given time, you can have either but not both of:
  - One mutable reference.
  - Any number of immutable references.
2. References must always be valid.


## Terms
- *stack*
- *heap*
- *pointer*
- *shallow copy* 
  - Copying the pointer, length, and capacity without copying the data.
  - Called a `move` in Rust
- *deep copy*
  - `clone()` in Rust
- *allocate* (memory)
- *free* (memory)
- *RAII*
  - *Resource Acquisition Is Initialization* is a pattern of deallocating resources at the end of an item's lifetime. (In C++)
  - `drop()`ing in Rust
- *reference*
- *slice*
  - *Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection.
