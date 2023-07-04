#[allow(unused_variables)]
#[allow(unused_assignments)]

pub fn example_loop() {
    let fruits = ["apples", "cake", "coffee"];

    for fruit in fruits {
        println!("I like {}.", fruit);
    }

    let numbers = [1, 2, 3];
    for &(mut number) in numbers.iter() {
        if number % 2 == 0 {
            println!("Even number: {}.", number);
        }
        number += 1;
        println!("Number after adding 1: {}.", number);
    }
}

/// At the same time:
/// - One or more references (&T) to a resource
/// - Exactly one mutable reference (&mut T)

pub fn example_ref() {
    let mut x = 10;

    // let rx = &mut x;
    // *rx += 10;

    let rx2: &mut i32 = &mut x;
    *rx2 += 10;

    println!("x is {}", rx2);
}
