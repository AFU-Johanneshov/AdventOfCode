#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Hash, Ord, Eq)]
pub struct VectorI16 {
    pub x: i16,
    pub y: i16,
}

use super::VectorF64;
use super::VectorI64;

impl_from_vectors!(VectorI16, VectorF64, VectorI64);
impl_base_from_tuples!(VectorI16);
impl_math_operators!(VectorI16, i16);

#[cfg(test)]
mod tests {
    use crate::vector::{vector_f64::VectorF64, vector_i16::VectorI16, vector_i64::VectorI64};
    use std::ops::{Add, Sub};

    create_integer_math_tests!(VectorI16, i16);

    create_from_test!(
        VectorI64,
        VectorI16,
        from_vector_i16,
        (42, -544),
        (42, -544)
    );

    // Converting from float to integer basically cuts out the decimals. Meaning there is no
    // rounding of the values.
    create_from_test!(
        VectorF64,
        VectorI16,
        from_vector_f64,
        (2384.2854, -15390.973),
        (2384, -15390)
    );
}
/*
impl VectorI16 {
    /// Returns true if the vector is outside a square between (x: 0, y: 0) and
    /// (x: size_override - 1, y: size_override - 1)
    pub fn out_of_bounds(&self, size_override: usize) -> bool {
        self.x < 0 || self.x >= size_override as i16 || self.y < 0 || self.y >= size_override as i16
    }

    /// Returns all 4 neighbours to this vector in the order of: Up (y+1), Right(x+1), Down(y-1), Left(x-1).
    pub fn neighbours(&self) -> [VectorI16; 4] {
        [*self + UP, *self + RIGHT, *self + DOWN, *self + LEFT]
    }

    /// Returns the 4 base direction vectors in the order of: Up (y+1), Right(x+1), Down(y-1), Left(x-1).
    pub fn directions() -> [VectorI16; 4] {
        [UP, RIGHT, DOWN, LEFT]
    }

    pub fn line_line_intersection(
        vector_a_source: VectorI16,
        vector_a: VectorI16,
        vector_b_source: VectorI16,
        vector_b: VectorI16,
    ) -> Option<VectorI16> {
        let Some(result) = VectorF64::line_line_intersection(
            VectorF64::from(vector_a_source),
            VectorF64::from(vector_a),
            VectorF64::from(vector_b_source),
            VectorF64::from(vector_b),
        ) else {
            return None;
        };

        match result.convert_to_vector_i16() {
            Ok(value) => Some(value),
            Err(error) => match error {
                VectorError::OutOfRange(_) => None,
            },
        }
    }
}*/
