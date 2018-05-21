# Error Handling

- `Result<T, E>` (for *recoverable* errors)
  - For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation.
- `panic!()` (for *unrecoverable* errors)
  - Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

## Unrecoverable Errors with `panic!`


## Recoverable Errors with `Result`
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Matching on Different Errors
### Shortcuts for Panic on Error: unwrap and expect
### Propagating Errors


## [To `panic!` or Not to `panic!`]()

