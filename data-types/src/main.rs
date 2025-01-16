use std::io;

fn main() {
    numeric();
    compounds();
    index_error();
}

fn numeric() {
    let addition = 423 + 355;
    println!("423 + 355 = {addition}");
    let subtraction = 123 - 456;
    println!("123 - 456 = {subtraction}");
    let multiplication = 34 * 123;
    println!("34 * 123 = {multiplication}");
    let division = 25.3 / 1.45;
    println!("25.3 / 1.45 = {division}");
    let int_division = 5 / 3;
    println!("Truncated 5 / 3 = {int_division}");
    let modulo = 29 % 6;
    println!("29 % 6 = {modulo}");
}

fn compounds() {
    let mut tup: (u32, bool) = (1000, true);
    let (a, b) = tup;
    println!("These are the values in the tuple: {a} and {b}");
    let c = tup.0;
    println!("This is how to access them directly: {c}");
    tup.1 = false;
    let (a, b) = tup;
    println!("Mutable tuples can have their elements changed: {a}, {b}");

    println!("The semicolon is useful to generate arrays of a certain size containing just a single element!");
    let arr: [u8; 10] = [3; 10];
    let first = arr[0];
    let last = arr[arr.len() -1];
    println!("arr = [3;10], arr[0] = {first}, arr[-1] = {last}");
}

fn index_error() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}