## [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/second-edition/ch08-02-strings.html)

Watch out for:
  1. Rust’s propensity for exposing possible errors 
  2. Strings being a more complicated data structure than many programmers give them credit for
  3. UTF-8

## What Is a String? (String Slices vs std::String)
- Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text. 
- Both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.
- owned and borrowed variants
### `str`
- Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. 
  - *string slices* are references to some UTF-8 encoded string data stored elsewhere. 
  - String literals are stored in the binary output of the program and are therefore string slices.
### `String`
- The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. 
- When Rustaceans refer to “strings” in Rust, they usually mean the `String` and the string slice `&str` types, not just one of those types. 

## Creating a New String
```rust
    let a = String::new();

    let data = "initial contents";
    let b = data.to_string();
    // the method also works on a literal directly:
    let c = "initial contents".to_string();

    let d = String::from("initial contents");
```

## Updating a String
- `+` (fn add(self, s: &str) -> String {...})
- `format!()`
```rust
    let mut a = String::new();
    a.push_str("pushed contents");

    let mut b = String::from("lo");
    b.push('l');
```

## Indexing into Strings
### Internal Representation
- A `String` is a wrapper over a `Vec<u8>`. 
### Bytes and Scalar Values and Grapheme Clusters! Oh My!
Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: 
- as bytes
- scalar values
- grapheme clusters (the closest thing to what we would call letters)
## Slicing Strings
You should use ranges to create string slices with caution, because doing so can crash your program.
## Methods for Iterating Over Strings
Be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
