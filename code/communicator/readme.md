# [`mod` and the Filesystem](https://doc.rust-lang.org/book/second-edition/ch07-01-mod-and-the-filesystem.html#module-definitions)

```
communicator
 ├── client
 └── network
     └── server
```

```
└── src
    ├── client.rs
    ├── lib.rs
    └── network
        ├── mod.rs
        └── server.rs
```

## Rules of Module Filesystems
- If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
- If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.
- These rules apply recursively, so if a module named foo has a submodule named bar and bar does not have submodules, you should have the following files in your src directory:
```
└── foo
    ├── bar.rs (contains the declarations in `foo::bar`)
    └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```
The modules should be declared in their parent module’s file using the mod keyword.


## Privacy Rules
Overall, these are the rules for item visibility:
- If an item is public, it can be accessed through any of its parent modules.
- If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.

```rust
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```
