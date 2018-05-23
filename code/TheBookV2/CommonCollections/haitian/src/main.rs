use std::collections::HashMap;

fn main()
{

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    //
    // // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify.
    // let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // This code will print each pair in an arbitrary order
    for (key, value) in &scores
    {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);


    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);

    println!("{:?}", scores);

    // =============================================================================

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
