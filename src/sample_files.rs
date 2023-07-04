use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};

#[allow(unused_assignments)]
#[allow(unused_variables)]

pub fn example_files() {
    // Create a file
    let mut file = File::create("src/example.txt").expect("create failed");

    // Write to a file
    file.write("Hello World!\n".as_bytes())
        .expect("write failed");

    // Append content to the file
    let mut file = OpenOptions::new()
        .append(true)
        .open("src/example.txt")
        .expect("can not open file");
    file.write_all("Adding content to the file\n".as_bytes())
        .expect("write failed");

    // Read from a file
    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Content of the file: {}", contents);

    // Remove file
    remove_file("src/example.txt").expect("delete failed");
}
