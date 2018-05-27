extern crate examplez;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // examplez::numberz::print(v);
    examplez::numberz::mean(&v);
    examplez::numberz::median(&v);
    examplez::numberz::mode(&v);


    examplez::pig_latin::oink();
    examplez::company::company();
}
