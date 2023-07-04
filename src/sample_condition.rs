use crate::sample_condition::Suite::{Club, Diamond, Heart, Spade};
use rand::Rng;

#[allow(unused_variables)]
#[allow(unused_assignments)]

pub fn example_condition() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 11);

    if num > 5 {
        println!("Number {} is greater than 5", num);
    } else if num < 5 {
        println!("Number {} is smaller than 5", num);
    } else {
        println!("Number {} is equal to 5", num);
    }

    let res = num >= 5;
    println!("Number is greater than or equal to 5: {}", res);
}

pub fn example_match() {
    print_choice(Heart);
    print_choice(Spade);
    print_choice(Club);
    print_choice(Diamond);

    country(44);
    country(84);
    country(125);
    country(-15);

    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    let point = (6, 0);
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y),
    }
}

enum Suite {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suite) {
    match choice {
        Heart => {
            println!("Heart")
        }
        Spade => {
            println!("Spade")
        }
        Club => {
            println!("Club")
        }
        Diamond => {
            println!("Diamond")
        }
    }
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        84 => "VN",
        1..=999 => "Unknown",
        _ => "Invalid",
    };

    println!("Country is {}", country)
}

fn get_oranges(amount: i32) -> &'static str {
    match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of",
    }
}
