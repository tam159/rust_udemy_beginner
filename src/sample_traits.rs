use std::ops::Add;

#[allow(unused_assignments)]
#[allow(unused_variables)]

struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awsome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello() {
        println!("Hello World!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello() {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awsome: awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello() {
        println!("System.out.println(\"Hello world!\");");
    }
}

pub fn example_traits() {
    let r = RustDev::new(true);
    let j = JavaDev::new(false);

    println!("{}", r.language());
    RustDev::say_hello();

    println!("{}", j.language());
    JavaDev::say_hello();
}

// Generics can be limited by traits
trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("{} barking", self.species)
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark())
}

pub fn example_trait_generics() {
    let dog = Dog {
        species: "retriever",
    };
    let _cat = Cat { color: "black" };

    bark_it(dog);
    // bark_it(cat);
}

// Returning traits
trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog { species: "pitbull" })
    } else {
        Box::new(Cat { color: "yello" })
    }
}

pub fn example_return_trait() {
    println!("The animal says {}", get_animal(0.5).make_noise());
    println!("The animal says {}", get_animal(2.1).make_noise());
}

// Adding traits to existing structures
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

pub fn example_vector_trait() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum())
    // let b = vec![1.0, 2.0, 3.0];
    // println!("sum float = {}", b.sum())
}

// Operator overloading
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn example_point_trait() {
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

// Static dispatch
trait Duplicatable {
    fn dupl(&self) -> String;
}

impl Duplicatable for String {
    fn dupl(&self) -> String {
        format!("{0}-{0}", *self)
    }
}

impl Duplicatable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicatable>(x: T) {
    println!("{}", x.dupl());
}

pub fn example_static_dispatch() {
    let a = 42;
    let b = "Hi Tam".to_string();
    duplicate(a);
    duplicate(b);
}

// Dynamic dispatch
fn duplicate2(x: &dyn Duplicatable) {
    println!("{}", x.dupl());
}

pub fn example_dynamic_dispatch() {
    let a = 42;
    let b = "Hi Tam".to_string();
    duplicate2(&a);
    duplicate2(&b);
}
