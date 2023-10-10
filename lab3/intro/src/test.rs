
fn modify_string(s: &mut String) {
    s.push_str(" world");
    s.push_str(GLOBAL.to_string().as_str());
}

static GLOBAL: i32 = 23;

struct Person {
    first_name: String,
    last_name: String,
}

// the output lifetime is the input lifetime
fn first_name(person: &Person) -> &String {
    &person.first_name
}

pub(crate) fn test_func() {
    println!("Hello from the test func");

    let mut s = String::from("hello");

    modify_string(&mut s);

    println!("{s}");

    // reference scope
    let b;
    let c;
    let m;
    {
        let t1 = 42;
        b = &t1;    // data is borrowed
        c = t1;     // data is copied
        let t2 = String::from("hey");
        m = t2;     // data is moved
    }
    // println!("{b}"); // t1 goes out of scope here, but b still has a reference to it
    println!("{c} {m}");

    // can somebody write behind our back
    let mut x = 32;
    let r = &mut x;
    // let r2 = &x; // mutable reference r cannot coexist with other refs
    *r += 2;
    x += 3;
    println!("x: {x}");

    // lifetimes
    let p1 = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };
    let name;
    {
        let p2 = Person {
            first_name: String::from("John"),
            last_name: String::from("Doe"),
        };
        // name = first_name(&p2); // borrowed value does not live long enough
        name = first_name(&p1);
    }
    println!("Name: {name}");

}