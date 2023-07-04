pub fn example_array() {
    let primes = [1, 2, 3, 4, 5];
    let _doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];

    println!("{:?}", primes);

    let numbers = [0; 15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 5];
    println!("{:?}", numbers[3]);
    numbers[3] = 5;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}
