#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 374;
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

This time we have a 2d map of space. The map contains empty spaces '.' and galaxies '#'. Our goal
is to calculate the sum of all the shortest paths between all galaxies.
However, there is a complication in that any row or column of the map that doesn't have a galaxy
is supposed to double in size.

This makes a solution using fixed arrays less feasible, but not impossible.
One way to do this is to load the map into a 2d array of tiles. Each tile is either empty or a
galaxy. To compensate for the double size we add a size field to the empty variant with a default
value of 1.

Then we make a function that iterates through all rows and columns of the map. Any row or column
found to only contain empty spaces has all its empty spaces size increased by one.

This should allow for a path calculating algorithm to know if a empty space is expanded or not.
However, I am not 100% sure this would actually work, and I would expect that there are flaws that
needs to be fixed.

But there is another way. Having a expanded 2d array is actually not needed here. Since what we
actually want to calculate is just the difference in position between galaxies, with their
positions adjusted for the row and column expansions.
This means we only need the 2d array to figure out which rows and columns are empty. We can then
simply "move" any galaxies that are after a empty row or column by one in either the x or y axis.

Once the galaxy list has been adjusted we just iterate through the list, getting the difference
between each galaxy and finally adding the differences together to get our result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
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
