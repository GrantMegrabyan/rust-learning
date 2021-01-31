struct Person {
    name: String,
    age: u32,
}

fn get_older_name<'a>(person1: &'a Person, person2: &'a Person) -> &'a String {
    if person1.age > person2.age {
        &person1.name
    } else {
        &person2.name
    }
}

fn single_lifetime() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 31,
    };

    let person2 = Person {
        name: String::from("Bob"),
        age: 33,
    };

    let older = get_older_name(&person1, &person2);

    println!("{} is older", older);
}

// Have to use two lifetimes
fn message_and_return<'a, 'b>(msg: &'a String, ret: &'b String) -> &'b String {
    println!("Printing the message: {}", msg);
    ret
}

fn foo(name: &String) -> &String {
    let msg = String::from("An awesome message");
    message_and_return(&msg, &name)
}

fn multiple_lifetimes() {
    let name = String::from("Alice");
    let ret = foo(&name);
    println!("Return value: {}", ret);
}

pub fn run() {
    println!("This is a Lifetimes example");

    single_lifetime();

    multiple_lifetimes();
}
