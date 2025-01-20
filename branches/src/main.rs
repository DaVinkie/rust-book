fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is divisible by 1.");
    }

    if_is_an_expression();
    let_me_try_something();
}


fn if_is_an_expression() {
    let condition = true;

    let x = if condition { 4 } else { 3 };  // Expression can be used as assignment.

    println!("The value of x is: {x}")
}


fn let_me_try_something() {
    let condition = true;
    let x;
    
    // x does not have to be mutable, because only 1 branch will be executed.
    if condition {
        x = 1;
        println!("Value of x = {x} inside if-else");
    } else {
        x = 2;
        println!("Value of x = {x} inside if-else");
    }

    {
        let x = 10;  // Shadowing is different because we reuse `let`
        println!("Shadowed value of x inside of new scope = {x}");    
    }
    // Value of x persists outside of if-else scope.
    println!("Value of x outside of if-else scope = {x}");
}