use crate::sample_enum::Colors::Green;
use crate::sample_enum::Person::Name;

pub fn example_enum() {
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Green;
    println!("{:?}", my_color);

    let person = Name(String::from("Tam"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(i32),
}
