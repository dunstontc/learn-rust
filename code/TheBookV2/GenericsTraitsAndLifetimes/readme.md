# [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/second-edition/ch10-00-generics.html#generic-types-traits-and-lifetimes)

<!-- TOC -->

- [Generic Types, Traits, and Lifetimes](#generic-types-traits-and-lifetimes)
  - [Generic Data Types](#generic-data-types)
    - [In Function Definitions](#in-function-definitions)
    - [In Struct Definitions](#in-struct-definitions)
    - [In Enum Definitions](#in-enum-definitions)
    - [In Method Definitions](#in-method-definitions)
    - [Performance of Code Using Generics](#performance-of-code-using-generics)
  - [Traits: Defining Shared Behavior](#traits-defining-shared-behavior)
    - [Defining a Trait](#defining-a-trait)
    - [Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
    - [Default Implementations](#default-implementations)
    - [Trait Bounds](#trait-bounds)
  - [Validating References with Lifetimes](#validating-references-with-lifetimes)
    - [Lifetimes Prevent Dangling References](#lifetimes-prevent-dangling-references)
    - [The Borrow Checker](#the-borrow-checker)
    - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
    - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
    - [Thinking in Terms of Lifetimes](#thinking-in-terms-of-lifetimes)
    - [Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
    - [Lifetime Elision Rules](#lifetime-elision-rules)
    - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
    - [The Static Lifetime](#the-static-lifetime)
  - [Generic Type Parameters, Trait Bounds, and Lifetimes Together](#generic-type-parameters-trait-bounds-and-lifetimes-together)

<!-- /TOC -->


## Generic Data Types
- We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. 

### In Function Definitions
```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
} 
```

### In Struct Definitions
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

### In Enum Definitions
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### In Method Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### Performance of Code Using Generics

---
## Traits: Defining Shared Behavior

### Defining a Trait

### Implementing a Trait on a Type
- One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to your crate.

### Default Implementations

### Trait Bounds

---
## Validating References with Lifetimes
- Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like most of the time types are inferred.   
- We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.   
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.  

### Lifetimes Prevent Dangling References
- The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

### The Borrow Checker
```rust
// An attempt to use a reference whose value has gone out of scope
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

```rust
//  A valid reference because the data has a longer lifetime than the reference
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

### Generic Lifetimes in Functions
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Lifetime Annotation Syntax
```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
- One lifetime annotation by itself doesn’t have much meaning because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
- For example, let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. 
- The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.

### Lifetime Annotations in Function Signatures
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Thinking in Terms of Lifetimes
- The way in which you need to specify lifetime parameters depends on what your function is doing. 
- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. 
- If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function. 
- Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

### Lifetime Annotations in Struct Definitions
- It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition. 
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

### Lifetime Elision Rules
The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error:
1. The first rule is that each parameter that is a reference gets its own lifetime parameter. 
    - In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.
2. The second rule is if there is exactly one *input lifetime* parameter, that lifetime is assigned to all *output lifetime* parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. The third rule is if there are multiple *input lifetime* parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all *output lifetime* parameters. 
    - This third rule makes methods much nicer to read and write because fewer symbols are necessary.

### Lifetime Annotations in Method Definitions

### The Static Lifetime

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
