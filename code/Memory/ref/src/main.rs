/// Nah
fn calculate_length_ob(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.
    return (s, length)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
// You can only have one mutable reference to a particular piece of data in a particular scope.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // This works without any problems. Ownership is moved out, and nothing is deallocated.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

}
