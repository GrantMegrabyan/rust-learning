use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let s = Arc::new(Mutex::new(String::from("Fearless")));

    let mut threads = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let inner = move || {
            let mut s = s.lock().unwrap();
            s.push('!');
            println!("{}", s);
        };
        threads.push(thread::spawn(inner));
    }

    for t in threads {
        let _ = t.join();
    }
}
