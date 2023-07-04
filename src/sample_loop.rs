#[allow(unused_variables)]
#[allow(unused_assignments)]

pub fn example_for() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i);
    }

    let pets = ["cat", "dog", "pig", "bear", "mouse"];
    for &pet in pets.iter() {
        if pet == "dog" {
            println!("{} barks too much", pet);
            continue;
        }
        if pet == "bear" {
            println!("{} is not a pet, break!", pet);
            break;
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..4).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square);
    }
}

pub fn example_while() {
    get_squares(20);
    get_cubes(100);
}

fn get_squares(limit: i32) {
    let mut x = 1;
    let mut square = x * x;

    while square < limit {
        println!("{0} * {0} = {1}", x, square);
        x += 1;
        square = x * x;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break;
        }
    }
}
