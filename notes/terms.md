# Terms

## Memory
  - **stack**
    - LIFO, speedy, quality Memory storage.
  - **heap**
    - Unordered, general, bulk Memory storage.
  - **data race**
    - A *data race* is similar to a race condition and happens when these three behaviors occur:
      1. Two or more pointers access the same data at the same time.
      2. At least one of the pointers is being used to write to the data.
      3. There’s no mechanism being used to synchronize access to the data.
  - **reference**
    - TODO
  - **lifetime**
    - Every reference in Rust has a *lifetime*, which is the scope for which that reference is valid. 
    - Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.
    - **Borrow Checker**
      - The Rust compiler has a *borrow checker* that compares scopes to determine that all borrows are valid.
    - **lifetime elision rules**
      - TODO
  - **deref coercion**
    - TODO
  - **unwinding**
    - TODO
## Functions
  - **statement**
    - *Statements* are instructions that perform some action and do not return a value.
  - **expression**
    - *Expressions* evaluate to a resulting value.
  - **associated function**
    - Static Method
## OOP (more or less)
  - **bounded parametric polymorphism**
    - TODO
  - **generics**
    - TODO
  - **trait**
    - A *trait* tells the Rust compiler about functionality a particular type has and can share with other types.
    - *Traits* are similar to a feature often called *interfaces* in other languages, although with some differences.
    - **Derivable Traits**
      - TODO
    - **trait bounds**
      - TODO
## Matches
  - **arm**
    - An *arm* consists of a pattern and the code that should be run if the value given to the beginning of the *match* expression fits that arm’s pattern.
  - **match guard**
    - an extra condition on a match arm that further refines the arm’s pattern
    - ex: `if error.kind() == ErrorKind::NotFound`
## Misc
  - **borrowing**
    - We call having references as function parameters borrowing. 
    - As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.
  - **shadowing**
    - This feature is often used in similar situations in which you want to convert a value from one type to another type.
  - **tuple**
    - A *tuple* is a general way of grouping together some number of other values with a variety of types into one compound type.
  - **tuple struct**
    - TODO
  - [**hashing function**](https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html#hashing-functions)
    - TODO
  - **Monomorphization**
    - *Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. 
    - Rust uses *monomorphization* to implement *generics* in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

