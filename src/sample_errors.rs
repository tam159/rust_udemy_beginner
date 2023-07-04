use std::fs::File;
use std::io;
use std::io::Read;

#[allow(unused_assignments)]
#[allow(unused_variables)]

pub fn example_errors() {
    // let v = vec![1, 2, 3];
    // v[10];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
    // panic!("Something went wrong. Can not proceed");

    // let f = File::open(("main.jpg")).unwrap();
    // let f = File::open(("main.jpg")).expect("Unable to open file");
    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("file not found\n{:?}", e);
        }
    }
    println!("Continuing on with the execution");

    divide(Some(1));
    divide(None);
    // divide(Some(0));

    // let a = read_username_from_file();
    let a = read_username_from_file_operator();
    println!("Username: {:?}", a);
    println!("Continuing on with the execution");
}

const ANSWER_TO_LIFE: i32 = 30;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Can not divide by 0"),
        Some(x) => println!("Result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("src/username2.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Instead of using above match statements, just need to open ? at the end to handle the error
fn read_username_from_file_operator() -> Result<String, io::Error> {
    let mut f = File::open("src/username2.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
