# [Testing](https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html)

<!-- TOC -->

  - [Writing Tests](#writing-tests)
    - [How to Write Tests](#how-to-write-tests)
    - [The Anatomy of a Test Function](#the-anatomy-of-a-test-function)
    - [Checking Results with the `assert!` Macro](#checking-results-with-the-assert-macro)
    - [Testing Equality with the `assert_eq!` and `assert_ne!` Macros](#testing-equality-with-the-assert_eq-and-assert_ne-macros)
    - [Adding Custom Failure Messages](#adding-custom-failure-messages)
    - [Checking for Panics with `should_panic`](#checking-for-panics-with-should_panic)
  - [Running Tests](#running-tests)
    - [Running Tests in Parallel or Consecutively](#running-tests-in-parallel-or-consecutively)
    - [Running a Subset of Tests by Name](#running-a-subset-of-tests-by-name)
    - [Filtering to Run Multiple Tests](#filtering-to-run-multiple-tests)
    - [Ignoring Some Tests Unless Specifically Requested](#ignoring-some-tests-unless-specifically-requested)
  - [Test Organization](#test-organization)
    - [Unit Tests](#unit-tests)
      - [The Tests Module and `#[cfg(test)]`](#the-tests-module-and-cfgtest)
      - [Testing Private Functions](#testing-private-functions)
    - [Integration Tests](#integration-tests)
      - [The tests Directory](#the-tests-directory)
      - [Submodules in Integration Tests](#submodules-in-integration-tests)
  - [Terms](#terms)

<!-- /TOC -->

## Writing Tests

### How to Write Tests
Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:
1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

### The Anatomy of a Test Function
- `assert_eq!()`
- `panic!()`
### Checking Results with the `assert!` Macro
### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
### Adding Custom Failure Messages
### Checking for Panics with `should_panic`


---
## Running Tests
### Running Tests in Parallel or Consecutively
### Running a Subset of Tests by Name
### Filtering to Run Multiple Tests
### Ignoring Some Tests Unless Specifically Requested


---
## Test Organization
### Unit Tests
#### The Tests Module and `#[cfg(test)]`
#### Testing Private Functions
### Integration Tests
#### The tests Directory
#### Submodules in Integration Tests


--- 
## Terms
-  integration tests
-  unit tests 
