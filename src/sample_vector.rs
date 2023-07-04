pub fn example_vector() {
    let _primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 4];

    println!("{:?}", primes);
    primes.push(7);
    println!("{:?}", primes);
    primes.remove(2);
    println!("{:?}", primes);

    let mut numbers = vec![2; 3];
    println!("{:?}", numbers);

    const DEFAULT: bool = true;
    let values = vec![DEFAULT; 8];
    println!("{:?}", values);

    numbers[1] = 8;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number * number);
    }
}
