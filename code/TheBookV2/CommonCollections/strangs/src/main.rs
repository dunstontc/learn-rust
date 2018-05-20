fn main() {
    let mut a = String::new();
    a.push_str("pushed contents");

    let data = "initial contents";
    let b = data.to_string();

    // the method also works on a literal directly:
    let c = "initial contents".to_string();

    let d = String::from("initial contents");

    let mut e = String::from("lo");
    e.push('l');

    let f1 = String::from("Hello, ");
    let f2 = String::from("world!");
    let f = f1 + &f2; // Note s1 has been moved here and can no longer be used
    // we can only add a `&str` to a `String`; we can’t add two `String` values together.

    let g1 = String::from("tic");
    let g2 = String::from("tac");
    let g3 = String::from("toe");
    let g = g1 + "-" + &g2 + "-" + &g3;

    let h1 = String::from("tic");
    let h2 = String::from("tac");
    let h3 = String::from("toe");
    let h = format!("{}-{}-{}", h1, h2, h3);


    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    println!("{}", g);
    println!("{}", h);

    for c in "hello".chars() { println!("{}", c); }
    for b in "hello".bytes() { println!("{}", b); }

    for c in "नमस्ते".chars() { println!("{}", c); }
    for b in "नमस्ते".bytes() { println!("{}", b); }
}
