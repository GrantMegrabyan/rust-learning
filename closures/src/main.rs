fn main() {
    let delim = "-".repeat(20);

    simple_closure();
    println!("{}", delim);

    closure_with_param();
    println!("{}", delim);

    capture_mutable();
    println!("{}", delim);

    closure_type();
    println!("{}", delim);

    forced_move_into_closure();
    println!("{}", delim);

    explicit_move();
    println!("{}", delim);

    exercise();
    println!("{}", delim);
}

fn simple_closure() {
    let msg = String::from("Hello");
    let say_message = || println!("{}", msg);

    say_message();
    say_message();
}

fn closure_with_param() {
    let msg = String::from("Welcome to a world of closures!");
    let greet = |name: &str| println!("Hello {}! {}", name, msg);

    greet("Alice");
    greet("Bob");
}

fn capture_mutable() {
    let mut count = 0;

    // Should declare as mutable if closure captures
    // a mutable variable
    let mut visit = || {
        count += 1;
        println!("Hello visitor #{}", count);
    };

    for _ in 1..5 {
        visit();
    }
}

fn closure_type() {
    let greet_closure = |greet: &str, name: &str| println!("{} from a closure, {}!", greet, name);

    greet_with_hiya(greet_closure, "Alice");
    greet_with_hiya(greet_fn, "Bob");
}

fn greet_with_hiya<F>(f: F, name: &str)
where
    F: Fn(&str, &str),
{
    f("Hiya", name)
}

fn greet_fn(greet: &str, name: &str) {
    println!("{} from a function, {}!", greet, name)
}

fn forced_move_into_closure() {
    let name = String::from("Alice");

    let welcome = || {
        // Forcing move
        let mut name = name;
        name += " and Bob";
        println!("Welcome, {}", name);
    };

    // Doesn't work because 'welcome' closure cannot be called
    // multiple times
    // call_five_times(welcome);

    // This works
    call_once(welcome);
}

fn call_five_times<F>(f: F)
where
    F: Fn(),
{
    for _ in 0..5 {
        f();
    }
}

fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// Using 'move' keyword

fn explicit_move() {
    // Implicit capture my reference
    let name = String::from("Alice");
    let say_hi = || println!("Hello, {}", name);
    call_fn(say_hi);
    call_fn_mut(say_hi);
    call_fn_once(say_hi);

    // Putting 'name' into a smaller scope.
    // This doesn't complile becase the closure captures 'name'
    // by reference which outlives the 'name' itself.
    // let say_hi = {
    //     let name = String::from("Bob");
    //     || println!("Hello, {}", name)
    // };

    // Same as before, but forcing 'name' to be moved into the closure
    // This wsy the closure can be called only once, because the value
    // is captured and consumed.
    let say_hi = {
        let name = String::from("Bob");
        || {
            let name_inner = name;
            println!("Hello, {}", name_inner)
        }
    };
    // call_fn(say_hi);
    // call_fn_mut(say_hi);
    call_fn_once(say_hi);

    // By using 'move' keyword the closure takes ownership of the value
    // but it is passed to the closure by reference which make it possible
    // to call it multiple times
    let say_hi = {
        let name = String::from("Bob");
        move || {
            println!("Hello, {}", name)
        }
    };
    // Here we need to pass the closure by reference otherwise
    // the ownership will be moved into the 'call_fn' not letting us 
    // use it again.
    call_fn(&say_hi);
    call_fn_mut(&say_hi);
    call_fn_once(&say_hi);
}

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f()
}

fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f()
}

fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f()
}

fn exercise() {
    let nums: Vec<u32> = (1..11).collect();

    for _ in 1..3 {
        for i in nums.iter().map(|n: &u32| n * 2) {
            println!("{}", i);
        }
    }
}