#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

const URL: &str = "google.com";

fn example_string() {
    //String slices, immutable, borrowed data
    let _cat: &str = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("cat: {}", cat);

    //String objects
    let _dog = String::new();
    let mut dog = String::from("Join");

    //Format
    let owner = format!("Hi I'm {} the owner of {}", "Tam", dog);
    println!("owner: {}", owner);

    //len, push and push_str
    println!("{}", dog.len());
    dog.push(' ');
    dog.push_str("the dog");

    //replace
    let _new_dog = dog.replace("the", "my");

    //split, split_whitespace, chars, .etc
    // Constant
    println!("URL: {}", URL);

    // Operators
    let a = 4 + 8;
    let b = 10 / 3;
    let c = 10 % 3;
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("{}", a > b);
    println!("{}", a > b && b < c);
}
