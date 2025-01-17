fn main() {
    let var: i32 = 5;
    println!("Hello, world!");
    another_function(var, 'm');
    let result = explanitory();
    println!("We played with return values: {result}");

}


fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

// Statements perform an action that doesn't result in a value
// Expressions evaluate into a resultant value

fn explanitory() -> i32 {
    let x: i32 = 5;  // Statement

    let y: i32 = {
        x + 10  // Expression, no semicolon ';'
    };  // Statement again

    y  // Expression - return statement
}  // Statement in and of itself.