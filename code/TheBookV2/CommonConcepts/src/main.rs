#![allow(unused_doc_comment)]

fn mutable() {
    let mut a: i32 = 5;
    println!("The value of a is: {}", a);
    a = 6;
    println!("The value of a is: {}", a);
}

fn constant() {
    /// ### let vs. const
    /// - Constants aren’t only immutable by default, they’re always immutable.
    /// - The type of the value must be annotated.
    /// - Constants can be declared in any scope, including the global scope
    ///   - This makes them useful for values that many parts of code need to know about.
    /// - Constants may only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    /// - Rust constant naming convention is to use all upper case with underscores between words.
    #[allow(dead_code)]
    const MAX_POINTS: u32 = 100_000;
}

fn shadowing() {
    // Rustaceans say that the first variable is shadowed by the second,
    // which means that the second variable’s value is what we’ll see when we use the variable.
    let b = 5;
    // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword.
    let b = b + 1;
    // This is different than marking a variable as mut, because unless we use the let keyword again,
    // we’ll get a compile-time error if we accidentally try to reassign to this variable.
    let b = b * 2;
    println!("The value of b is: {}", b);

    // Say our program asks a user to show how many spaces
    // they want between some text by inputting space characters,
    // but we really want to store that input as a number:
    let _spaces = "   ";
    let _spaces = _spaces.len();

    // This won't work because you can't mutate a variable's type:
    // let mut spaces = "   ";
    // spaces = spaces.len();
}

// In function signatures, you must declare the type of each parameter.
fn multiple_params(c: i32, d: i32) {
    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);
}

fn scope() {
    let x = 1;
    println!("The value of x is: {}", x);

    let y = {
        let x = 2;
        println!("The value of x is: {}", x);
        x + 3
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}

// If you add a semicolon to the end of an expression,
// you turn it into a statement, which will then not return a value.
fn five() -> i32 {
    5
}

fn truth() {
    let number = 3;

    // if number {
    //     println!("number was three");
    // }
    // No truthy/falsey values in Rust :(
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn conditionally() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

fn lift() {
    let e = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", e[index]);
        index = index + 1;
    }

    let f = [60, 70, 80, 90, 100];
    for element in f.iter() {
        println!("the value is: {}", element);
    }

    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number = number - 1;
    // }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


fn main() {
    mutable();
    constant();
    shadowing();
    multiple_params(24, 48);
    scope();
    five();
    let v = five();
    println!("The value of v is: {}", v);
    truth();
    conditionally();
    lift();
}
