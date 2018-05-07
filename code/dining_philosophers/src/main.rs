use std::{thread, time};
use std::sync::{Mutex, Arc};

struct Philosopher {
    name : String, // Generally speaking, working with a type which owns its data is easier than working with one that uses references.
    left : usize,
    right: usize,
}

impl Philosopher {
    // This name, `new()`, isn’t anything special to Rust, but it is a convention for functions that create new instances of structs.
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name : name.to_string(),
            left : left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let two_seconds = time::Duration::from_millis(2000);

        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(two_seconds);
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks : vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Bill S. Preston Esq.", 0, 1),
        Philosopher::new("Ted \"Theodore\" Logan", 1, 2),
        Philosopher::new("Socrates", 2, 3),
        Philosopher::new("Joan of Arc", 3, 4),
        Philosopher::new("Abraham Lincoln", 0, 4), // This is what prevents deadlock: one of our philosophers is left handed!
    ];

    // This creates an iterator that takes ownership of each philosopher.
    // We need to do this to pass them to our threads.
    // We take that iterator and call map on it,
    // which takes a closure as an argument and calls that closure on each element in turn.
    let handles: Vec<_> = philosophers // The _ is a type placeholder.
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
