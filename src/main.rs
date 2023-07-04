mod player;
#[allow(dead_code)]
mod sample_string;
mod sample_variable;

#[allow(unused_imports)]
use std::io;

mod archive;
mod sample_array;
mod sample_condition;
mod sample_enum;
mod sample_errors;
mod sample_files;
mod sample_generic;
mod sample_high_order_fn;
mod sample_loop;
mod sample_macro;
mod sample_memory;
mod sample_reference;
mod sample_slice;
mod sample_structure;
mod sample_threads;
mod sample_traits;
mod sample_tuple;
mod sample_vector;

use crate::archive::arch::arch_file as arc;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

static mut R: i32 = 0;

/// Create document
/// What is this module trying to achieve

#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    //! # Main function
    //!
    //! ```
    //! fn main()
    //! ```
    //!
    //! Reads user input then print
    let mut input = String::new();

    // Print a message to the user
    println!("Say hello");
    println!(
        "My name is {0} and I'm {1} years old and I'm {0} again",
        "Tam", 30
    );
    println!(
        "My name is {name} and I'm {age} years old and I'm {name} again",
        name = "Tam",
        age = 30
    );
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    println!("Array: {:?}", [1, 2, 3]); // Debug, convert complex type to string to print

    /*
    Check response
    and act
     */
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
    //         println!("You said {}", input);
    //     }
    //     Err(e) => {
    //         println!("Something went wrong {}", e);
    //     }
    // }

    for _i in 1..3 {
        say_hi();
    }

    let mut name = "Tam";
    let greeting = say_hello(&mut name);
    println!("Greeting: {}", greeting);
    println!("Hi {}", name);

    player::play_movie("snatch.mp4");
    player::play_audio("music.mp3");

    clean::perform_clean();
    clean::files::clean_files();

    // archive::arch::arch_file("some files need to be deleted")
    arc("some files need to be deleted");

    let mut rng = thread_rng();
    let a: i32 = rng.gen();
    println!("Random number: {}", a);
    println!("Random bounded integer: {}", rng.gen_range(0, 100));
    println!("Random bounded float: {}", rng.gen_range(0.0, 100.0));

    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    println!("Generated string: {}", rand_string);

    sample_array::example_array();
    sample_vector::example_vector();
    sample_slice::example_slice();
    sample_tuple::example_tuple();
    sample_structure::example_structure();
    sample_enum::example_enum();
    sample_generic::example_generic();
    sample_condition::example_condition();
    sample_condition::example_match();
    sample_loop::example_for();
    sample_loop::example_while();
    sample_reference::example_loop();
    sample_reference::example_ref();

    // Scope
    {
        let a = 3;
        println!("a = {}", a);
    }
    println!("a = {}", a);

    // Global variable
    unsafe {
        R = 3;
        println!("R = {}", R);
    }

    // Closures
    |a: i32, b: i32| println!("{}", a + b);
    let sum = |a: i32, b: i32| -> i32 { a + b };
    println!("sum = {}", sum(2, 3));

    let add_one = |a: i32| a + 1;
    println!("{}", add_one(6));

    let add_one = |b: i32| -> i32 { b + 1 };
    println!("{}", add_one(7));

    let gen = |x| println!("x = {}", x);
    // gen(3);
    gen(true);

    sample_high_order_fn::example_high_order_fn();

    my_macro!();
    name!("Tam");
    multiple_name!("Tam", "Hoa");
    xy!(x => 5);
    xy!(y => 3*9);
    build_fn!(hey);
    hey();

    sample_traits::example_traits();
    sample_traits::example_trait_generics();
    sample_traits::example_return_trait();
    sample_traits::example_vector_trait();
    sample_traits::example_point_trait();
    sample_traits::example_static_dispatch();
    sample_traits::example_dynamic_dispatch();
    sample_memory::example_memory();
    sample_memory::example_lifetime();
    sample_memory::example_reference_counted();
    sample_files::example_files();
    sample_errors::example_errors();
    sample_threads::example_threads();
    sample_threads::example_channel();
    sample_threads::example_multiple_channels();
    sample_threads::example_mutex();
}

fn say_hi() {
    println!("Say hi")
}

fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    *name = "Hoa";
    println!("Hello {}", name);
    greeting
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning HDD");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
