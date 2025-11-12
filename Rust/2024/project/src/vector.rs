#[macro_use]
mod vector_macros;
pub use vector_f64::VectorF64;
pub use vector_i16::VectorI16;
pub use vector_i64::VectorI64;

mod vector_f64;
mod vector_i16;
mod vector_i64;

pub trait Vector {}

pub enum VectorError {
    OutOfRange,
}

// TODO: Implement more standard traits.
// Ord, Hash etc

//pub use vector_i16::VectorI16;

//pub enum VectorError {
//    OutOfRange(String),
//}

/*
const UP: VectorI16 = VectorI16 { x: 0, y: 1 };
const DOWN: VectorI16 = VectorI16 { x: 0, y: -1 };
const RIGHT: VectorI16 = VectorI16 { x: 1, y: 0 };
const LEFT: VectorI16 = VectorI16 { x: -1, y: 0 };
*/
//impl_from_vectors!(VectorF64, VectorI16, VectorI64);

/*
macro_rules! experiment1small {
    ($from:tt, $to:tt) => {
        print!("from: ");
        print!($($from)*);
        print!(" to: ");
        print!($($to)*);
    };
}

macro_rules! experiment1 {
    ($from:tt, $ ($to:tt),*) => {
        $(
            experiment1small!($from);
        )*
    };
}*/

/*
macro_rules! testonly_println {
    ($($x:tt)*) => {
        #[cfg(test)]
        println!($($x)*);
    };
}

macro_rules! testonly_print {
    ($($x:tt)*) => {
        #[cfg(test)]
        print!($($x)*);
    };
}
macro_rules! impl_from_vectors {
    ( $( $from:ty ),* ) => {

        $(
            impl_from_singular_vector!($from, VectorF64);
        )*
    };
}
*/

#[cfg(test)]
mod tests {
    /*


    macro_rules! impl_all_vector_conversions {
        ($($from:ty),*) => {
            $(
                $(
                    impl_from_vector!($from, $to);
                )*
            )*

        };

         * #[test]
        fn line_line_intersection_out_of_range_none() {
            let vector_a = VectorI16 { x: 3, y: 4 };
            let vector_a_origin = VectorI16::default();
            let vector_b = VectorI16 { x: 1, y: -1 };
            let vector_b_origin = VectorI16 { x: 2, y: 2 };
            let result =
                VectorI16::line_line_intersection(vector_a_origin, vector_a, vector_b_origin, vector_b);

            if let Some(value) = result {
                panic!(
                    "Expected result was None but Some({:?}) was received",
                    value
                );
            }
        }

        #[test]
        fn line_line_intersection_in_range_some() {
            let vector_a = VectorI16 { x: 94, y: 34 };
            let vector_a_origin = VectorI16::default();
            let vector_b = VectorI16 { x: -22, y: -67 };
            let vector_b_origin = VectorI16 { x: 8400, y: 5400 };

            let expected_result: VectorI16 = VectorI16 { x: 7520, y: 2720 };

            let result =
                VectorI16::line_line_intersection(vector_a_origin, vector_a, vector_b_origin, vector_b);

            let Some(value) = result else {
                panic!(
                    "Expected result was Some({:?}) but None was received",
                    expected_result
                );
            };
            assert_eq!(value, expected_result);

            let vector_a = VectorI16 { x: 17, y: 86 };
            let vector_a_origin = VectorI16::default();
            let vector_b = VectorI16 { x: -84, y: -37 };
            let vector_b_origin = VectorI16 { x: 7870, y: 6450 };

            let expected_result: VectorI16 = VectorI16 { x: 646, y: 3268 };

            let result =
                VectorI16::line_line_intersection(vector_a_origin, vector_a, vector_b_origin, vector_b);

            let Some(value) = result else {
                panic!(
                    "Expected result was Some({:?}) but None was received",
                    expected_result
                );
            };
            assert_eq!(value, expected_result);
        }*/
}
