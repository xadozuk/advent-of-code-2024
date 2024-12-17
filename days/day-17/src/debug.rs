pub const DEBUG: bool = false;

macro_rules! debugln {
    () => {
        if crate::debug::DEBUG {
            println!()
        }
    };
    ($($arg:tt)*) => {{
        if crate::debug::DEBUG {
            println!($($arg)*);
        }
    }};
}

pub(crate) use debugln;
