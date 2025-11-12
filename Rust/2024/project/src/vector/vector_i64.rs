#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct VectorI64 {
    pub x: i64,
    pub y: i64,
}

use super::VectorF64;
use super::VectorI16;

impl_from_vectors!(VectorI64, VectorI16, VectorF64);
impl_base_from_tuples!(VectorI64);
impl_math_operators!(VectorI64, i64);

#[cfg(test)]
mod tests {
    use crate::vector::{vector_f64::VectorF64, vector_i16::VectorI16, vector_i64::VectorI64};
    use std::ops::{Add, Sub};

    create_integer_math_tests!(VectorI64, i64);

    create_from_test!(
        VectorI16,
        VectorI64,
        from_vector_i16,
        (42, -544),
        (42, -544)
    );

    // Converting from float to integer basically cuts out the decimals. Meaning there is no
    // rounding of the values.
    create_from_test!(
        VectorF64,
        VectorI64,
        from_vector_f64,
        (241295.2854, -542344.973),
        (241295, -542344)
    );
}

/*
impl From<(usize, usize)> for VectorI64 {
    /// Converts two usize to a vector. NOTE: Currently vector uses i16 and might not be able to
    /// fit the full range of usize.
    fn from(values: (usize, usize)) -> Self {
        Self {
            x: values.0 as i64,
            y: values.1 as i64,
        }
    }
}*/

/*
impl VectorI64 {
    /// Returns true if the vector is outside a square between (x: 0, y: 0) and
    /// (x: size_override - 1, y: size_override - 1)
    pub fn out_of_bounds(&self, size_override: usize) -> bool {
        self.x < 0 || self.x >= size_override as i64 || self.y < 0 || self.y >= size_override as i64
    }

    /// Returns all 4 neighbours to this vector in the order of: Up (y+1), Right(x+1), Down(y-1), Left(x-1).
    pub fn neighbours(&self) -> [VectorI64; 4] {
        todo!() // Create a VectorI64 from VectorI16 function first.
                /*[
                    *self + UP,
                    *self + RIGHT,
                    *self + DOWN,
                    *self + LEFT,
                ]*/
    }

    /*
    /// Returns the 4 base direction vectors in the order of: Up (y+1), Right(x+1), Down(y-1), Left(x-1).
    pub fn directions() -> [VectorI16; 4] {
        [UP, RIGHT, DOWN, LEFT]
    }*/

    /*
    pub fn line_line_intersection(
        vector_a_source: VectorI64,
        vector_a: VectorI64,
        vector_b_source: VectorI64,
        vector_b: VectorI64,
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
    }*/
}*/
