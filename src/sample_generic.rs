use crate::sample_generic::Colors::Red;

pub fn example_generic() {
    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 6.6, y: 8.8 };

    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Red("#f00");
    let c2 = Red("255");

    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3: Point2<i32, f64> = Point2 { x: 6, y: 8.8 };
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Blue(T),
    Green(T),
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V,
}
