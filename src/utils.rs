#[allow(unused_macros)]
#[macro_export]
macro_rules! debug_println {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}
