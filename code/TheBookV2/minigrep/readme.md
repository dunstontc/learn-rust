# [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/stable/book/second-edition/ch12-00-an-io-project.html)

## Usage

```shell
cargo run searchstring example-filename.txt 
```

## Reading the Argument Values

> The args Function and Invalid Unicode
> Note that `std::env::args` will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead. That function returns an iterator that produces `OsString` values instead of `String` values. 

## Reading a File

## Refactoring to Improve Modularity and Error Handling
1. As a function gains responsibilities, it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts. It’s best to separate functionality so each function is responsible for one task.
2. It’s best to group the configuration variables into one structure to make their purpose clear.
3. It would be best if all the error-handling code were in one place so future maintainers had only one place to consult in the code if the error-handling logic needed to change. 
4. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

### Separation of Concerns for Binary Projects

The responsibilities that remain in the main function after this process should be limited to the following:

Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error

### Extracting the Argument Parser


### Grouping Configuration Values

> The Trade-Offs of Using clone
> There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost.

### Creating a Constructor for Config

### Fixing the Error Handling

#### Improving the Error Message

