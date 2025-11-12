#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct VectorF64 {
    pub x: f64,
    pub y: f64,
}

use super::VectorI16;
use super::VectorI64;

impl_from_vectors!(VectorF64, VectorI16, VectorI64);
impl_base_from_tuples!(VectorF64);
impl_math_operators!(VectorF64, f64);

#[cfg(test)]
mod tests {
    use crate::vector::{vector_f64::VectorF64, vector_i16::VectorI16, vector_i64::VectorI64};
    use std::ops::{Add, Sub};

    create_floating_point_math_tests!(VectorF64, f64);

    create_from_test!(
        VectorI16,
        VectorF64,
        from_vector_i16,
        (42, -544),
        (42.0, -544.0)
    );

    create_from_test!(
        VectorI64,
        VectorF64,
        from_vector_i64,
        (241295, -542344),
        (241295.0, -542344.0)
    );
    //create_impl_from_test!(VectorF64, VectorI64);
    //create_from_implementation_tests!(VectorF64);
    /*
    #[test]
    fn add_operator() {
        let a = VectorF64 { x: 35.0, y: -85.0 };
        let b = VectorF64 { x: 67.0, y: 94.0 };
        assert_eq!(VectorF64 { x: 102.0, y: 9.0 }, a + b);
    }*/

    /*
    create_addition_test!(
        add_operator,
        VectorF64,
        (35.0, -85.0),
        (67.0, 94.0),
        (102.0, 9.0)
    );*/
}
/*
impl VectorF64 {
    pub fn convert_to_vector_i16(self) -> Result<VectorI16, VectorError> {
        if self.x > i16::MAX as f64 || self.y > i16::MAX as f64 {
            return Err(VectorError::OutOfRange(String::from(
                "Vector {self:?} is to large to fit inside a VectorI16",
            )));
        }
        let x = self.x as i16;
        let y = self.y as i16;
        Ok(VectorI16 { x, y })
    }

    pub fn line_line_intersection(
        vector_a_source: VectorF64,
        vector_a: VectorF64,
        vector_b_source: VectorF64,
        vector_b: VectorF64,
    ) -> Option<VectorF64> {
        /*let vector_a_origin: VectorF64 = VectorF64::default();
        let vector_b_origin: VectorF64 = vector_b_source - vector_a_source;

        let t2 = (vector_a.x * vector_b_origin.y - vector_b_origin.x * vector_a.y)
            / (vector_b.x * vector_a.y - vector_b.y * vector_a.x);
        let t1 = (vector_b_origin.x + t2 * vector_b.x) / vector_a.x;*/

        let (t1, t2) = Self::line_line_intersection_get_scalars(
            vector_a_source,
            vector_a,
            vector_b_source,
            vector_b,
        );

        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }

        let result = vector_a * t1 + vector_a_source;

        println!("vector_a_source: {:?}, \nvector_a: {:?}, \nvector_b_source: {:?}, \nvector_b: {:?}, \nt1: {}, \nt2: {}, \nresult: {:?}, \n", vector_a_source, vector_a, vector_b_source, vector_b, t1, t2, result);

        Some(result)
    }

    pub fn line_line_intersection_get_scalars(
        vector_a_source: VectorF64,
        vector_a: VectorF64,
        vector_b_source: VectorF64,
        vector_b: VectorF64,
    ) -> (f64, f64) {
        let vector_a_origin: VectorF64 = VectorF64::default();
        let vector_b_origin: VectorF64 = vector_b_source - vector_a_source;

        let t2 = (vector_a.x * vector_b_origin.y - vector_b_origin.x * vector_a.y)
            / (vector_b.x * vector_a.y - vector_b.y * vector_a.x);
        let t1 = (vector_b_origin.x + t2 * vector_b.x) / vector_a.x;

        (t1, t2)
    }
    // Formula:
    // Step one:
    // Vector A: (Va)
    // (x,y) = t1 * (Va_x, Va_y) where t1 is greater than or equal to 0. t1 is a scalar.
    //
    // Vector B: (Vb)
    // (x,y) = P0 + t2 * (Vb_x, Vb_y) where t2 is grater than or equal to 0.
    // Here P0 is the starting point of B. t2 is a scalar.
    //
    // Step two:
    // To get the intersecting point the coordinates (x,y) of A and B must be quual.
    // Giving us this:
    // t1 * (Va_x, Va_y) = P0 + t2 * (Vb_x, Vb_y)
    //
    // Then split them into two equations, one for x and one for y.
    // t1 * Va_x = P0.x + t2 * Vb_x
    // t1 * Va_y = P0.y + t2 * Vb_y
    //
    // Then rearange to solve for t1 and t2.
    // (Since t1 * Va_x = ... we can rearrange it to t1 = .../Va_x)
    // t1 = (P0.x + t2 * Vb_x) / Va_x
    //
    //
    // Solve for t2:
    // t2 = (Va_x * P0.y - P0.x * Va_y) / (Vb_x * Va_y - Vb_y * Va_x)
    //
    // Solve for t1:
    // t1 = (P0.x + t2 * Vb_x) / Va_x
    //
    // If t1 < 0 the intersection point is behind the origin of vector A. So not valid.
    // If t2 < 0 the intersecting point is behind the origin of vector B. So not valid.
    // Else: The two vectors does intersect. The point of intersection can then be calculated by:
    // (x, y) = t1 * (Va_x, Va_y) = (P0.x, P0.y) + t2 * (Vb_x, Vb_y)
}*/
