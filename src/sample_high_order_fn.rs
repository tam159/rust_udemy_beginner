#[allow(unused_assignments)]
#[allow(unused_variables)]

pub fn example_high_order_fn() {
    let square = |a: i32| a * a;
    apply(square, 6);

    // Calculate the sum of all the squares less than 500 only for even numbers
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("The sum is {}", sum);

    // With HOFs
    let sum2: i32 = (0..)
        .map(|x: i32| x * x)
        .take_while(|x| x < &limit)
        .filter(|x| is_even(*x))
        .sum();

    println!("The sum with HOFs is {}", sum2);
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}
