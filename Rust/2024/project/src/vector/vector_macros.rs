macro_rules! impl_add_and_subtract {
    ($type:ty) => {
        impl std::ops::Add for $type {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl std::ops::Sub for $type {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
    };
}

macro_rules! impl_scalar_multiplication {
    ($type:ty, $part_type:ty) => {
        impl std::ops::Mul<$part_type> for $type {
            type Output = Self;

            fn mul(self, rhs: $part_type) -> Self::Output {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
    };
}

/// Implements the basic math operators.
///
/// type: The vector type which will get the implementations.
/// part_type: The type of the vectors x and y fields.
/// Example:
/// impl_math_operators!(VectorI16, i16);
macro_rules! impl_math_operators {
    ($type:ty, $part_type:ty) => {
        impl_add_and_subtract!($type);
        impl_scalar_multiplication!($type, $part_type);
    };
}

macro_rules! create_math_test {
    ($test_name:ident, $operation:ident, $type:ident, $vector_a:tt, $vector_b:tt, $result:tt) => {
        #[test]
        fn $test_name() {
            let a = $type {
                x: $vector_a.0,
                y: $vector_a.1,
            };
            let b = $type {
                x: $vector_b.0,
                y: $vector_b.1,
            };
            let verification = $type {
                x: $result.0,
                y: $result.1,
            };
            assert_eq!(a.$operation(b), verification);
        }
    };
}

macro_rules! create_integer_math_tests {
    ($vectortype:ident, $part_type:ty) => {
        // Addition tests.
        create_math_test!(
            math_add_positive,
            add,
            $vectortype,
            (35, -85),
            (67, 74),
            (102, -11)
        );
        create_math_test!(
            math_add_negative,
            add,
            $vectortype,
            (42, -3),
            (-25, -76),
            (17, -79)
        );
        create_math_test!(
            math_add_past_zero,
            add,
            $vectortype,
            (42, -3),
            (-75, 76),
            (-33, 73)
        );

        // Subtraction tests.
        create_math_test!(
            math_sub_positive,
            sub,
            $vectortype,
            (62, -24),
            (23, 19),
            (39, -43)
        );
        create_math_test!(
            math_sub_negative,
            sub,
            $vectortype,
            (62, -24),
            (-23, -19),
            (85, -5)
        );
        create_math_test!(
            math_sub_past_zero,
            sub,
            $vectortype,
            (62, -24),
            (83, -39),
            (-21, 15)
        );
    };
}

macro_rules! create_floating_point_math_tests {
    ($vectortype:ident, $part_type:ty) => {
        // Addition tests.
        create_math_test!(
            math_add_positive,
            add,
            $vectortype,
            (35.3, -85.63),
            (67.9, 74.88),
            (103.2, -10.75)
        );
        create_math_test!(
            math_add_negative,
            add,
            $vectortype,
            (42.8, -3.829),
            (-25.31, -76.45),
            (17.49, -80.279)
        );
        create_math_test!(
            math_add_past_zero,
            add,
            $vectortype,
            (42.42, -3.89),
            (-75.31, 76.45),
            (-32.89, 72.56)
        );

        // Subtraction tests.
        create_math_test!(
            math_sub_positive,
            sub,
            $vectortype,
            (62.823, -24.7),
            (23.95, 19.32),
            (38.873000000000005, -44.019999999999996)
        );
        create_math_test!(
            math_sub_negative,
            sub,
            $vectortype,
            (62.823, -24.7),
            (-23.95, -19.32),
            (86.773, -5.379999999999999)
        );
        create_math_test!(
            math_sub_past_zero,
            sub,
            $vectortype,
            (62.823, -24.7),
            (83.95, -39.32),
            (-21.127000000000002, 14.620000000000001)
        );
    };
}

/*
62.823, -24.7
-23.95, -19.32
86.773, -5.379999999999999

62.823, -24.7
83.95, -39.32
-21.127000000000002, 14.620000000000001
*/

macro_rules! impl_from_tuples {
    ($to:ty, $( $from:ty ),* ) => {
        $(
            impl From<$from> for $to {
                fn from(tuple: $from) -> Self {
                    Self {
                        x: tuple.0 as _,
                        y: tuple.1 as _,
                    }
                }
            }
        )*
    };
}

/// Implements from<(_, _)> (tuple of two primitive numeric variables) for the specified vector.
///
///
macro_rules! impl_base_from_tuples {
    ($to:ty) => {
        impl_from_tuples!(
            $to,
            (usize, usize),
            (f32, f32),
            (f64, f64),
            (u8, u8),
            (u16, u16),
            (u32, u32),
            (u64, u64),
            (i8, i8),
            (i16, i16),
            (i32, i32),
            (i64, i64)
        );
    };
}

macro_rules! impl_from_single_vector {
    ($to:ty, $from:ty) => {
        impl From<$from> for $to {
            fn from(vector: $from) -> Self {
                Self {
                    x: vector.x as _,
                    y: vector.y as _,
                }
            }
        }
    };
}

/// Implements from<> for vectors.
///
/// First argument is the vector which will get the implementations.
/// All arguments after the first will be separate from<argument> implementations.
///
/// Example: ...!(VectorF64, VectorI16, VectorI64);
/// Expands into:
/// impl From<Vectori16> for VectorF64 {...}
/// impl From<Vectori64> for VectorF64 {...}
macro_rules! impl_from_vectors {
    ($to:ty, $( $from:ty ),* ) => {
        $(
            impl_from_single_vector!($to, $from);
        )*
    };
}

macro_rules! create_from_test {
    ($from:ident, $to:ident, $test_name:ident, $source:tt, $expected:tt) => {
        #[test]
        fn $test_name() {
            let source = $from {
                x: $source.0,
                y: $source.1,
            };
            let expected = $to {
                x: $expected.0,
                y: $expected.1,
            };
            assert_eq!($to::from(source), expected);
        }
    };
}

/*
macro_rules! create_impl_from_test {
    ($from:ty, $to:ty) => {
        #[test]
        fn $test_name() {
            print!("From {} to {}", stringify!($from), stringify!($to));
            todo!();
        }
    };
}

macro_rules! create_from_implementation_tests {
    ($to:ty) => {
        create_impl_from_test!(VectorF64, $to);
        create_impl_from_test!(VectorI16, $to);
        create_impl_from_test!(VectorI64, $to);
    };
}*/
