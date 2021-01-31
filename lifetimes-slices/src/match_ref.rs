#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn print_person(person: Person) {
    // This works
    println!("Full Person value: {:?}", person);
    match person.name {
        Some(name) => println!("Name is {}", name),
        None => println!("Name is not provided"),
    }

    match person.age {
        Some(age) => println!("Age is {}", age),
        None => println!("Age is not provided"),
    }
    // This won't work
    // println!("Full Person value: {:?}", person);
}

fn print_person_with_borrow(person: Person) {
    match &person.name {
        Some(name) => println!("Name is {}", name),
        None => println!("Name is not provided"),
    }

    match person.age {
        Some(age) => println!("Age is {}", age),
        None => println!("Age is not provided"),
    }
    // Now this works
    println!("Full Person value: {:?}", person);
}

fn print_person_with_ref(person: Person) {
    match person.name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("Name is not provided"),
    }

    match person.age {
        Some(age) => println!("Age is {}", age),
        None => println!("Age is not provided"),
    }
    // Now this works
    println!("Full Person value: {:?}", person);
}

fn print_person_on_birthday_borrow(mut person: Person) {
    match person.name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("Name is not provided"),
    }

    match &mut person.age {
        Some(age) => {
            println!("Age is {}", age);
            *age += 1;
        },
        None => println!("Age is not provided"),
    }
    println!("Full Person value: {:?}", person);
}

fn print_person_on_birthday_ref(mut person: Person) {
    match person.name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("Name is not provided"),
    }

    match person.age {
        Some(ref mut age) => {
            println!("Age is {}", age);
            *age += 1;
        },
        None => println!("Age is not provided"),
    }
    println!("Full Person value: {:?}", person);
}

pub fn run () {
    println!("This is Example 1!");

    print_person(Person {
        name: Some(String::from("Alice")),
        age: None,
    });

    print_person_with_borrow(Person {
        name: Some(String::from("Alice")),
        age: None,
    });

    print_person_with_ref(Person {
        name: Some(String::from("Alice")),
        age: None,
    });

    print_person_on_birthday_borrow(Person{
        name: Some(String::from("Bob")),
        age: Some(31),
    });

    print_person_on_birthday_ref(Person{
        name: Some(String::from("Bob")),
        age: Some(31),
    });
}