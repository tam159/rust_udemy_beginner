#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

fn example_varialbe() {
    let name = "Alex";
    let mut age = 32;
    let amount: i64 = 123456789123456;

    age = 43;

    println!("age: {}", age);

    let color = "blue";
    let color = 86;

    println!("color: {}", color);

    let (a, b, c) = (12, 15, "haha");

    let pi: f32 = 4.0;
    let million = 1_000_000;
    println!("million: {}", million);

    let is_day = true;
    let is_night = false;
    println!("day: {}, night: {}", is_day, is_night);

    let char1 = 'a';
    let smiley_face = '\u{1F601}';
    println!("smiley_face: {}", smiley_face);
}
