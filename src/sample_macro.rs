#[allow(unused_assignments)]
#[allow(unused_variables)]
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("First macro")
    };
}

#[macro_export]
macro_rules! name {
    ($name: expr) => {
        println!("Hey {}", $name)
    };
}

#[macro_export]
macro_rules! multiple_name {
    ($($name: expr), *) => {
        $(println!("Hello {}", $name);)*
    }
}

#[macro_export]
macro_rules! xy {
    (x => $e: expr) => {
        println!("X is {}", $e)
    };
    (y => $e: expr) => {
        println!("Y is {}", $e)
    };
}

#[macro_export]
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    };
}
