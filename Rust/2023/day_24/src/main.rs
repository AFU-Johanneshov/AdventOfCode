#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 2;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 25433;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have a list where each row contains the starting position and velocity of a particle in 3D
space. Our goal is to figure out how many of the particles paths will intersect within a specific
area. For part one we are meant to only use the x and y axis, ignoring z.

I am thinking the way to do this is just to go through all the particles, calculating if and where
their paths meet. Then if said point is inside the test area simply add 1 to a counter.

Once all particles has been checked we should have our answer.

Note: The test area differs between the example and full data. 7 to 27 for the example, and
200000000000000 to 400000000000000 for the full data.
*/
mod part_one {
    use crate::reader;
    use std::{collections::btree_set::Intersection, error::Error};

    #[derive(Default, Debug, PartialEq)]
    struct Vector3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vector3D {
        fn new(x: f64, y: f64, z: f64) -> Vector3D {
            Self { x, y, z }
        }

        fn is_past(&self, target: f64) -> bool {
            self.x > target || self.y > target // || self.z > target
        }
        fn is_before(&self, target: f64) -> bool {
            self.x < target || self.y < target // || self.z < target
        }
    }

    #[derive(Default, Debug)]
    struct Particle {
        position: Vector3D,
        velocity: Vector3D,
    }

    impl Particle {
        fn parse(particle_str: &str) -> Result<Particle, Box<dyn Error>> {
            let values = particle_str
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != 6 {
                return Err(format!(
                    "Line: [{}] contained {} values instead of the expected 6!",
                    particle_str,
                    values.len()
                )
                .into());
            }
            Ok(Particle {
                position: Vector3D::new(values[0], values[1], values[2]),
                velocity: Vector3D::new(values[3], values[4], values[5]),
            })
        }

        fn intersection_with(&self, other: &Particle) -> Result<Vector3D, bool> {
            //println!("\nParticle A: {:?}", self);
            //println!("Particle B: {:?}", other);
            let determinant =
                (self.velocity.x * other.velocity.y) - (self.velocity.y * other.velocity.x);
            if determinant == 0.0 {
                //println!("Is parallel!");
                return Err(true);
            }

            let scaling_factor = ((other.position.x - self.position.x) * other.velocity.y
                - (other.position.y - self.position.y) * other.velocity.x)
                / determinant;

            //println!("scaling_factor: {scaling_factor}");

            let x = self.position.x + self.velocity.x * scaling_factor;
            let y = self.position.y + self.velocity.y * scaling_factor;

            //println!("point: x:{}, y:{}", x, y);

            if scaling_factor < 1.0 || negative_scaling_factor(other, self) {
                //println!("scaling_factor is less than 0!");
                return Err(false);
            }

            Ok(Vector3D::new(x, y, 0.0))
        }
    }

    fn negative_scaling_factor(a: &Particle, other: &Particle) -> bool {
        let determinant = (a.velocity.x * other.velocity.y) - (a.velocity.y * other.velocity.x);
        if determinant == 0.0 {
            return false;
        }

        let scaling_factor = ((other.position.x - a.position.x) * other.velocity.y
            - (other.position.y - a.position.y) * other.velocity.x)
            / determinant;
        scaling_factor < 1.0
    }

    fn get_test_area() -> (f64, f64) {
        #[cfg(test)]
        let (lower, upper) = (7.0, 27.0);
        #[cfg(not(test))]
        let (lower, upper) = (200000000000000.0, 400000000000000.0);
        (lower, upper)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let particles = reader::get_lines(data_path)?
            .map(|line| Particle::parse(&line))
            .collect::<Result<Vec<_>, _>>()?;

        let mut intersections = Vec::new();

        for i in 0..particles.len() {
            //let mut parallel_count = 0;
            for a in i + 1..particles.len() {
                if let Ok(intersection) = particles[i].intersection_with(&particles[a]) {
                    intersections.push(intersection);
                }
                /*
                match particles[i].intersection_with(&particles[a]) {
                    Ok(intersection) => intersections.push(intersection),
                    Err(is_parallel) => {
                        if is_parallel {
                            parallel_count += 1;
                        }
                    }
                } // */
            }
            //println!("parallel_count: {parallel_count}");
        }

        let (lower_test_area, upper_test_area) = get_test_area();

        let mut result = 0;
        for intersection in intersections {
            //println!("Intersection: {:?}", intersection);
            if !intersection.is_before(lower_test_area) && !intersection.is_past(upper_test_area) {
                //println!("Intersection: {:?} Is inside the area!", intersection);
                result += 1;
            }
        }

        //Err("Error wooooooo!".into())
        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");

    if cfg!(feature = "bench") {
        println!("Benchmarks are enabled!\n");
    }

    println!("\nPart One {}\n", {
        match benchmark!("calculate", { part_one::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
    println!("\nPart Two {}\n", {
        match benchmark!("calculate", { part_two::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
}
