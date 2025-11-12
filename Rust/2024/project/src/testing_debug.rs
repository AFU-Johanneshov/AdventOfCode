#[macro_export]
macro_rules! testonly_println {
    ($($x:tt)*) => {
        #[cfg(test)]
        println!($($x)*);
    };
}

#[macro_export]
macro_rules! testonly_print {
    ($($x:tt)*) => {
        #[cfg(test)]
        print!($($x)*);
    };
}
