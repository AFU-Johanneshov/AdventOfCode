//#[cfg(debug_assertions)]
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 2;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have a list of shapes that can be flipped and rotated. And another list where each row holds
the size of a grid and how many of each shape needs to be placed in it.
The task is to figure out which row is valid and which is not.
A row is valid if all the shapes required fits inside the grid without overlap.

I could try to brute force it but it won't be very efficient, and I am not even sure if the
calculation time would make it a real option. It is very possible it would take ages.
But at the same time I can't see what other alternative I have at this moment.

And seeing how the grids in the full data can be VERY big I really doubt a brute force solution
will ever work.

So, can the problem be divided into parts?
One issue with that would the the testdata. The grid and shape count is much smaller so proper
testing will be troublesome.

One way could be to try and pre-calculate which combinations of shapes creates a perfect square.
Then use that as a base to place a set of shapes in the most efficient way possible.

Or maybe we could make "educated" guesses if they fit or not. If estimated correctly it might
be good enough to be correct all the time. Maybe...

*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    const SHAPECOUNT: usize = 6;

    #[derive(Clone, Copy, Default, Debug)]
    pub struct Shape {
        grid: [[bool; 3]; 3],
    }

    impl Shape {
        fn parse(_data_lines: &[Option<String>]) -> Result<Shape, Box<dyn Error>> {
            todo!();
        }
    }

    struct Region {
        gridsize: (usize, usize),
        required_shapes: [u8; SHAPECOUNT],
    }

    impl Region {
        fn parse(_data_string: &str) -> Result<Region, Box<dyn Error>> {
            todo!();
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;

        let mut shapes = [Shape::default(); SHAPECOUNT];
        for shape_index in 0..SHAPECOUNT {
            lines.next(); // Skip shape index line.
            shapes[shape_index] = Shape::parse(&[lines.next(), lines.next(), lines.next()])?;
            lines.next(); // Skip empty separator line.
        }

        for s in shapes {
            s.print();
            //Shape::temp();
        }

        let mut regions: Vec<Region> = Vec::new();
        for line in lines {
            regions.push(Region::parse(&line)?);
        }

        Err("NotImplemented: This problem has not been solved yet!".into())
    }

    //

    //

    // Debug-Only code
    // -------------------------------------------------------------------------------------------
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    mod debug {
        use super::Shape;

        #[deprecated(note = "Debug-only method; guard with #[cfg(debug_assertions)]")]
        impl Shape {
            pub fn print(&self) {
                let mut grid = String::new();
                for r in self.grid {
                    for b in r {
                        grid.push(if b { '#' } else { '·' });
                    }
                    grid.push('\n');
                }
                println!("{}", grid);
            }
        }
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

    //

    //

    // Debug-Only code
    // -------------------------------------------------------------------------------------------
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    mod debug {
        //#[deprecated(note = "Debug-only method; guard with #[cfg(debug_assertions)]")]
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    print!("Running Program...\n\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}\n", value),
        Err(err) => println!("FAILED with error:\n{}\n", err),
    }
}
