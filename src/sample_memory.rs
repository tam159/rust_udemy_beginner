use std::rc::Rc;

#[allow(unused_assignments)]
#[allow(unused_variables)]

pub fn example_memory() {
    let i = 5;
    let j = i;
    // Copy value 5 to j, so both i and j are owners of 2 different values
    println!("i = {}", i);
    println!("j = {}", j);

    let v = vec![1, 2, 3, 4, 5];
    let w = v;
    // Ownership is transferred from v to w, to we can not print v
    println!("w = {:?}", w);
    // println!("v = {:?}", v);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };

    // Ownership is transferred back to v
    let v = foo(w);
    println!("v = {:?}", v);
    // println!("w = {:?}", w);

    let mut a = 6;
    let b = &mut a;
    // println!("De-reference b = {}", *b);
    // println!("a = {}", a);
    *b += 2;
    // b have NOT release the borrow, so we can not use a here
    // If we wanna use a, we need to release the borrow by NOT using b after using a
    // a += 2;
    // println!("a = {}", a);
    println!("De-reference b = {}", *b);

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("element in vector v: {}", i);
        // Cannot borrow `v` as mutable because it is also borrowed as immutable
        // It guarantee use the memory safety, it we can add element to v, it would be an infinity loop
        // v.push(6);
    }
}

fn get_str() -> &'static str {
    "Hello"
}

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person, // Define the same lifetime for both Dog and Person
                       // to make sure that the person is valid as long as the dog exists
}

impl Person {
    // fn get_name<'l> (&'l self) -> &'l String {
    // Lifetime elision - compiler builds lifetimes for us when evident
    // so the above declaration is equivalent to the below
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn example_lifetime() {
    println!("Example string: {}", get_str());

    let p1 = Person {
        name: String::from("John"),
    };
    let d1 = Dog {
        name: String::from("Max"),
        owner: &p1,
    };

    println!("Example dog: {:?}", d1);

    let person_name: &String;
    // let mut person_name: &String;
    {
        let _p2 = Person {
            name: String::from("Mary"),
        };
        // person_name = p2.get_name();
        // error[E0597]: `p2` does not live long enough

        person_name = p1.get_name();
    }
    println!("person_name: {}", person_name)
}

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand }
    }
    fn drive(&self) {
        println!("{} is driving", &self.brand)
    }
}

pub fn example_reference_counted() {
    let brand = Rc::new(String::from("BMW"));
    println!("Pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("Pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}", brand);
    println!("Pointers: {}", Rc::strong_count(&brand));
}
