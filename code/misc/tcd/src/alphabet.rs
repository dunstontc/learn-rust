pub fn the_alphabet() {
    // An iterator over the lowercase alpha's
    let ascii_iter = (0..26).map(|x| (x + 'a' as u8) as char);

    println!("{:?}", ascii_iter.collect::<Vec<_>>());
}
