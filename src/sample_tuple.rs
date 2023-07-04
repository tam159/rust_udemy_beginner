pub fn example_tuple() {
    let mut person = ("Tam", 30, true);
    println!("{:?}", person);
    person.0 = "Hoa";
    println!("{:?}", person.0);

    let (name, age, employment) = person;
    println!("name: {}, age: {}, employed: {}", name, age, employment);
}
