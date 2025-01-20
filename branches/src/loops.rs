fn main() {
    loop_loop();
    while_loop();
    for_loop();
    for_range_loop();
}


fn loop_loop() {
    let mut count = 0;
    
    let result = 'outer_loop: loop{
        let mut x = 0;
        println!("Outer count = {count}");

        loop {
            println!("I am looping!");
            println!("I did {x} small loops");
            if count == 5 {
                break 'outer_loop count;  // labeled loop
            }   
            if x > 6 {
                break;  // current loop
            }
            x += 1; 
        }
        count += 1;
    };
    println!("I did {result} big loops");
}


fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}


fn for_range_loop() {
    for number in (1..4).rev() {  // reverse
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}