fn main() {
    // To create a new, empty vector, we can call the `Vec::new()` function.
    // let v: Vec<i32> = Vec::new();

    // It’s more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro for convenience.
    {
        let _v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let second: &i32 = &v[1];
    let third: Option<&i32> = v.get(2);

    println!("{:?}", second);
    println!("{:?}", third);
}
