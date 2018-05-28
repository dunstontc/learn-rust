# [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/second-edition/ch10-00-generics.html#generic-types-traits-and-lifetimes)


## Traits: Defining Shared Behavior

### Defining a Trait

### Implementing a Trait on a Type
- One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to your crate.

### Default Implementations

### Trait Bounds

## Validating References with Lifetimes
> Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like most of the time types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### Lifetimes Prevent Dangling References

### The Borrow Checker

### Generic Lifetimes in Functions

### [Lifetime Annotation Syntax](https://doc.rust-lang.org/book/second-edition/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax)

### Lifetime Annotations in Function Signatures

### Thinking in Terms of Lifetimes

### Lifetime Annotations in Struct Definitions

### Lifetime Elision

### Lifetime Annotations in Method Definitions

### The Static Lifetime

### Generic Type Parameters, Trait Bounds, and Lifetimes Together
