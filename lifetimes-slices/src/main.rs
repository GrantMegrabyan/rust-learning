mod lifetimes;
mod match_ref;
mod single_iterator;

fn main() {
    match_ref::run();
    println!("{}", "-".repeat(20));

    single_iterator::run();
    println!("{}", "-".repeat(20));

    lifetimes::run();
}
