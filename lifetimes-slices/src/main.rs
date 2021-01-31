mod match_ref;
mod single_iterator;
mod lifetimes;

fn main() {
    match_ref::run();
    println!("{}", "-".repeat(20));

    single_iterator::run();
    println!("{}", "-".repeat(20));

    lifetimes::run();
}
