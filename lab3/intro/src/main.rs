mod test;

fn do_swap() {
    let mut a = String::from("apple");
    let mut b = String::from("bread");
    println!("Before swap: a = {a}, b = {b}");
    std::mem::swap::<String>(&mut a, &mut b);
    println!("After swap: a = {a}, b = {b}");
}

fn smallest_first(a: &mut i32, b: &mut i32) {
    if a <= b {
        return;
    }
    std::mem::swap::<i32>(a, b);
}

fn sorted(names: &[String]) -> Vec<String> {
    let mut sorted_vector = vec![];

    for name in names {
        sorted_vector.insert(0, name.clone());
    }

    sorted_vector.sort();
    return sorted_vector
}

fn main() {
    test::test_func();

    do_swap();

    let mut a = 3;
    let mut b = 5;
    smallest_first(&mut a, &mut b);
    println!("a: {a}");

    let people = [String::from("John Doe"), String::from("Jane Eyre"), String::from("Jane Doe")];
    let sorted_people = sorted(&people);
    for name in sorted_people {
        print!("{name}\n")
    }
}
