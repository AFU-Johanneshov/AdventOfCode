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
        fn parse(data_lines: &[String]) -> Result<Shape, Box<dyn Error>> {
            if data_lines.len() != 3 {
                return Err(format!(
                    "E1: Shape::parse(): incorrect data_lines length! Expected 3 but received {})",
                    data_lines.len()
                )
                .into());
            }

            let mut grid = [[false; 3]; 3];

            for line_index in 0..3 {
                let mut data_string = data_lines[line_index].chars();
                for i in 0..3 {
                    let Some(c) = data_string.next() else {
                        return Err(format!("E2: Shape::parse(): data_line {} has too few characters! Expected 3 but received [{}])"
                                , line_index, data_lines[line_index]).into());
                    };

                    match c {
                        '#' => grid[line_index][i] = true,
                        '.' => {} // False is default so nothing needs to be done.
                        _ => {
                            return Err(format!(
                            "E3: Shape::parse(): Received an invalid character [{}] in line [{}]",
                            c, data_lines[line_index]
                        )
                            .into())
                        }
                    }
                }
            }

            Ok(Shape { grid })
        }
    }

    struct Region {
        gridsize: (usize, usize),
        required_shapes: [u8; SHAPECOUNT],
    }

    impl Region {
        fn parse(data_string: &str) -> Result<Region, Box<dyn Error>> {
            let mut data_strings = data_string.split(": ");
            let (Some(grid_size), Some(shape_requirements)) =
                (data_strings.next(), data_strings.next())
            else {
                return Err(format!(
                    "E4: Region::parse(): Data string [{}] has a incorrect format!",
                    data_string
                )
                .into());
            };

            let mut grid_axis = grid_size.split("x");
            let (Some(x), Some(y)) = (grid_axis.next(), grid_axis.next()) else {
                return Err(format!(
                    "E5: Region::parse(): Data string [{}] has a incorrect format!",
                    data_string
                )
                .into());
            };

            let mut shapes = shape_requirements.split(" ");
            let mut required_shapes = [0; SHAPECOUNT];
            for i in 0..SHAPECOUNT {
                let Some(s) = shapes.next() else {
                    return Err(format!(
                        "E5: Region::parse(): Data string [{}] has a incorrect format!",
                        data_string
                    )
                    .into());
                };

                required_shapes[i] = s.parse()?;
            }

            Ok(Region {
                gridsize: (x.parse()?, y.parse()?),
                required_shapes,
            })
        }
    }

    fn take_three_lines(
        lines: &mut impl Iterator<Item = String>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::new();
        for i in 0..3 {
            let Some(s) = lines.next() else {
                return Err(format!("E5: take_three_lines(): Not enough remaining lines in iterator! Managed to take {} but expected 3!", i).into());
            };
            result.push(s);
        }

        Ok(result)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;

        let mut shapes = [Shape::default(); SHAPECOUNT];
        for shape in shapes.iter_mut().take(SHAPECOUNT) {
            lines.next(); // Skip shape index line.
            *shape = Shape::parse(&take_three_lines(&mut lines)?)?;
            lines.next(); // Skip empty separator line.
        }

        let mut regions: Vec<Region> = Vec::new();
        for line in lines {
            regions.push(Region::parse(&line)?);
        }

        for s in shapes {
            s.print();
        }
        for r in regions {
            r.print();
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

        use super::Region;

        #[deprecated(note = "Debug-only method; guard with #[cfg(debug_assertions)]")]
        impl Region {
            pub fn print(&self) {
                let mut result = String::new();
                result.push_str(&format!("{}x{}:", self.gridsize.0, self.gridsize.1));
                for value in self.required_shapes {
                    result.push_str(&format!(" {}", value));
                }
                println!("{}", result);
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
