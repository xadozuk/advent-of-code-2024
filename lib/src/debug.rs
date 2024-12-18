#[macro_export]
macro_rules! debugln {
    () => {
        if cfg!(debug_assertions) {
            println!()
        }
    };
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions)  {
            println!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug {
    () => {
        if cfg!(debug_assertions) {
            print!()
        }
    };
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions)  {
            print!($($arg)*);
        }
    }};
}
