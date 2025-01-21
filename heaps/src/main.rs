fn main() {
    let a = Box::new(14);
    let b = a; // Value is borrowed, a no longer points to anything.
    let c = Box::new(15);

    println!("Values of {b} and {c}.");

    let word = String::from("Daniël");
    let full = add_suffix(word);
    println!("Full name: {full}");

    this_works();
}

/* Ownership of `word` is passed to the function and the variable
is no longer valid. */

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}


fn this_works() {
    let b = Box::new(0);
    let b2 = b;
    // println!("{}", b);
    // This works because even though ownership from b is moved to b2,
    // b is not freed yet until after the function call.

    // Actually, this does NOT work but it is TECHNICALLY safe behavior.
    move_a_box(b2);
}


fn move_a_box(b: Box<i32>) {
    let b2 = b;
    println!("{b2}");
}
